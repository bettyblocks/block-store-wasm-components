use chrono::{DateTime, Datelike, FixedOffset, Months};

use crate::exports::betty_blocks::datetime::datetime::{Guest, OffsetSize, Timestamp};

wit_bindgen::generate!({ generate_all });

enum TimeOffset {
    TimeDelta(chrono::TimeDelta),
    Months(i32),
}

impl TimeOffset {
    fn new(offset_count: i32, offset_size: OffsetSize) -> Self {
        match offset_size {
            OffsetSize::Seconds => Self::TimeDelta(chrono::TimeDelta::seconds(offset_count as i64)),
            OffsetSize::Minutes => Self::TimeDelta(chrono::TimeDelta::minutes(offset_count as i64)),
            OffsetSize::Hours => Self::TimeDelta(chrono::TimeDelta::hours(offset_count as i64)),
            OffsetSize::Days => Self::TimeDelta(chrono::TimeDelta::days(offset_count as i64)),
            OffsetSize::Weeks => Self::TimeDelta(chrono::TimeDelta::weeks(offset_count as i64)),
            OffsetSize::Months => Self::Months(offset_count),
            OffsetSize::Years => Self::Months(offset_count * 12),
        }
    }

    fn add_months(
        timestamp: chrono::DateTime<FixedOffset>,
        offset_count: i32,
    ) -> Result<chrono::DateTime<FixedOffset>, String> {
        if offset_count < 0 {
            timestamp
                .checked_sub_months(Months::new(offset_count.unsigned_abs()))
                .ok_or_else(|| {
                    String::from("Could not offset the datetime by the specified amount")
                })
        } else {
            timestamp
                .checked_add_months(Months::new(offset_count as u32))
                .ok_or_else(|| {
                    String::from("Could not offset the datetime by the specified amount")
                })
        }
    }

    fn offset_time(
        &self,
        timestamp: chrono::DateTime<FixedOffset>,
    ) -> Result<chrono::DateTime<FixedOffset>, String> {
        match self {
            Self::TimeDelta(time_delta) => {
                timestamp.checked_add_signed(*time_delta).ok_or_else(|| {
                    String::from("Could not offset the datetime by the specified amount")
                })
            }
            Self::Months(months) => Self::add_months(timestamp, *months),
        }
    }

    fn offset_time_in_business_days(
        &self,
        timestamp: chrono::DateTime<FixedOffset>,
        offset_size: OffsetSize,
    ) -> Result<DateTime<FixedOffset>, String> {
        if let TimeOffset::TimeDelta(time_delta) = self
            && offset_size < OffsetSize::Weeks
        {
            // If you're only counting business days every week is 5 days instead of 7, so we add 2 days for every 5 in the offset we started with.
            let amount_of_days = time_delta.num_days();
            let amount_of_non_business_days = amount_of_days / 5 * 2;
            let offset_timestamp = timestamp
                .checked_add_signed(
                    *time_delta + chrono::TimeDelta::days(amount_of_non_business_days),
                )
                .ok_or_else(|| {
                    String::from("Could not offset the datetime by the specified amount")
                })?;

            let offset_timestamp_weekday = offset_timestamp.weekday().num_days_from_monday();

            // If the day of the week comes before the original day of the week, that means we passed a weekend we didn't yet account for.
            // If the day of the week ends up on saturday or sunday (5 or 6) we are currently in a weekend we didn't yet account for.
            if offset_timestamp_weekday < timestamp.weekday().num_days_from_monday()
                || offset_timestamp_weekday > 4
            {
                offset_timestamp
                    .checked_add_days(chrono::Days::new(2))
                    .ok_or_else(|| {
                        String::from("Could not offset the datetime by the specified amount")
                    })
            } else {
                Ok(offset_timestamp)
            }
        } else {
            self.offset_time(timestamp)
        }
    }
}

struct Component;

impl Guest for Component {
    fn now() -> Timestamp {
        chrono::Utc::now().to_rfc3339()
    }

    fn change_timezone(timestamp: Timestamp, timezone: String) -> Result<Timestamp, String> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp)
            .map_err(|_| String::from("The timestamp is not correctly formatted"))?;
        let timezone: FixedOffset = timezone
            .parse()
            .map_err(|_| String::from("The timezone is not correctly formatted"))?;

        Ok(timestamp.with_timezone(&timezone).to_rfc3339())
    }

    fn offset_datetime(
        timestamp: Timestamp,
        offset_count: i32,
        offset_size: OffsetSize,
    ) -> Result<Timestamp, String> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp)
            .map_err(|_| String::from("The timestamp is not correctly formatted"))?;

        let time_offset = TimeOffset::new(offset_count, offset_size);

        time_offset
            .offset_time(timestamp)
            .map(|offset_timestamp| offset_timestamp.to_rfc3339())
    }

    fn offset_datetime_in_business_days(
        timestamp: Timestamp,
        offset_count: i32,
        offset_size: OffsetSize,
    ) -> Result<Timestamp, String> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp)
            .map_err(|_| String::from("The timestamp is not correctly formatted"))?;

        let time_offset = TimeOffset::new(offset_count, offset_size);

        time_offset
            .offset_time_in_business_days(timestamp, offset_size)
            .map(|offset_timestamp| offset_timestamp.to_rfc3339())
    }
}

export! {Component}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    #[test]
    fn change_timezone_with_invalid_timestamp_test() {
        assert_eq!(
            Component::change_timezone(String::new(), String::from("Z"))
                .unwrap_err()
                .as_str(),
            "The timestamp is not correctly formatted"
        );
    }

    #[test]
    fn change_timezone_with_invalid_timezone_test() {
        assert_eq!(
            Component::change_timezone(String::from("1970-01-02T00:00:00+00:00"), String::new())
                .unwrap_err()
                .as_str(),
            "The timezone is not correctly formatted"
        );
    }

    #[test]
    fn change_timezone_validity_test() {
        let now = chrono::Utc::now().fixed_offset();
        let now_in_another_timezone = chrono::DateTime::parse_from_rfc3339(
            &Component::change_timezone(now.to_rfc3339(), String::from("+01:00")).unwrap(),
        )
        .unwrap();

        // The timezone is 3600 seconds offset from UTC.
        assert_eq!(now_in_another_timezone.timezone().local_minus_utc(), 3600);

        // The time is still the same when converted back to UTC.
        assert_eq!(now.naive_utc(), now_in_another_timezone.naive_utc());
    }

    #[test]
    fn offset_datetime_large_timedelta_validity_test() {
        // Offset the UNIX EPOCH by 3 days, 3 hours, 3 minutes and 3 seconds exactly
        let offset_datetime = DateTime::parse_from_rfc3339(
            &Component::offset_datetime(
                String::from("1970-01-01T00:00:00+00:00"),
                270183,
                OffsetSize::Seconds,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(offset_datetime.day(), 4);
        assert_eq!(offset_datetime.hour(), 3);
        assert_eq!(offset_datetime.minute(), 3);
        assert_eq!(offset_datetime.second(), 3);
    }

    #[test]
    fn offset_datetime_in_business_days_weekend_skip_test() {
        assert_eq!(
            Component::offset_datetime_in_business_days(
                String::from("1970-01-02T00:00:00+00:00"),
                1,
                OffsetSize::Days
            )
            .unwrap()
            .as_str(),
            "1970-01-05T00:00:00+00:00"
        );
    }

    #[test]
    fn offset_datetime_in_business_days_weekend_skip_with_hour_offset_test() {
        assert_eq!(
            Component::offset_datetime_in_business_days(
                String::from("1970-01-02T06:00:00+00:00"),
                18,
                OffsetSize::Hours
            )
            .unwrap()
            .as_str(),
            "1970-01-05T00:00:00+00:00"
        );
    }

    #[test]
    fn offset_datetime_in_business_days_multiple_weekend_skip_test() {
        assert_eq!(
            Component::offset_datetime_in_business_days(
                String::from("1970-01-02T00:00:00+00:00"),
                14,
                OffsetSize::Days
            )
            .unwrap()
            .as_str(),
            "1970-01-22T00:00:00+00:00"
        );
    }
}
