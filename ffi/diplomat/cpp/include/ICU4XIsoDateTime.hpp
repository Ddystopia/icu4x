#ifndef ICU4XIsoDateTime_HPP
#define ICU4XIsoDateTime_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XIsoDateTime.h"

class ICU4XIsoDateTime;
#include "ICU4XError.hpp"
class ICU4XIsoDate;
class ICU4XTime;
class ICU4XDateTime;
class ICU4XCalendar;
#include "ICU4XIsoWeekday.hpp"

/**
 * A destruction policy for using ICU4XIsoDateTime with std::unique_ptr.
 */
struct ICU4XIsoDateTimeDeleter {
  void operator()(capi::ICU4XIsoDateTime* l) const noexcept {
    capi::ICU4XIsoDateTime_destroy(l);
  }
};

/**
 * An ICU4X DateTime object capable of containing a ISO-8601 date and time.
 * 
 * See the [Rust documentation for `DateTime`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html) for more information.
 */
class ICU4XIsoDateTime {
 public:

  /**
   * Creates a new [`ICU4XIsoDateTime`] from the specified date and time.
   * 
   * See the [Rust documentation for `new_iso_datetime`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime) for more information.
   */
  static diplomat::result<ICU4XIsoDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

  /**
   * Creates a new [`ICU4XIsoDateTime`] from an [`ICU4XIsoDate`] and [`ICU4XTime`] object
   * 
   * See the [Rust documentation for `new`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new) for more information.
   */
  static ICU4XIsoDateTime new_from_date_and_time(const ICU4XIsoDate& date, const ICU4XTime& time);

  /**
   * Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
   * 
   * See the [Rust documentation for `from_minutes_since_local_unix_epoch`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch) for more information.
   */
  static diplomat::result<ICU4XIsoDateTime, ICU4XError> from_minutes_since_local_unix_epoch(int32_t minutes);

  /**
   * Gets the date contained in this object
   * 
   * See the [Rust documentation for `date`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date) for more information.
   */
  ICU4XIsoDate date() const;

  /**
   * Gets the time contained in this object
   * 
   * See the [Rust documentation for `time`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time) for more information.
   */
  ICU4XTime time() const;

  /**
   * Converts this to an [`ICU4XDateTime`] capable of being mixed with dates of
   * other calendars
   * 
   * See the [Rust documentation for `to_any`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any) for more information.
   */
  ICU4XDateTime to_any() const;

  /**
   * Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
   * 
   * See the [Rust documentation for `minutes_since_local_unix_epoch`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch) for more information.
   */
  int32_t minutes_since_local_unix_epoch() const;

  /**
   * Convert this datetime to one in a different calendar
   * 
   * See the [Rust documentation for `to_calendar`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar) for more information.
   */
  ICU4XDateTime to_calendar(const ICU4XCalendar& calendar) const;

  /**
   * Returns the hour in this time
   * 
   * See the [Rust documentation for `hour`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.hour) for more information.
   */
  uint8_t hour() const;

  /**
   * Returns the minute in this time
   * 
   * See the [Rust documentation for `minute`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.minute) for more information.
   */
  uint8_t minute() const;

  /**
   * Returns the second in this time
   * 
   * See the [Rust documentation for `second`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.second) for more information.
   */
  uint8_t second() const;

  /**
   * Returns the nanosecond in this time
   * 
   * See the [Rust documentation for `nanosecond`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.nanosecond) for more information.
   */
  uint32_t nanosecond() const;

  /**
   * Returns the 1-indexed day in the month for this date
   * 
   * See the [Rust documentation for `day_of_month`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_month) for more information.
   */
  uint32_t day_of_month() const;

  /**
   * Returns the day in the week for this day
   * 
   * See the [Rust documentation for `day_of_week`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_week) for more information.
   */
  ICU4XIsoWeekday day_of_week() const;

  /**
   * Returns 1-indexed number of the month of this date in its year
   * 
   * See the [Rust documentation for `month`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month) for more information.
   */
  uint32_t month() const;

  /**
   * Returns the year number for this date
   * 
   * See the [Rust documentation for `year`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.year) for more information.
   */
  int32_t year() const;

  /**
   * Returns the number of months in the year represented by this date
   * 
   * See the [Rust documentation for `months_in_year`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.months_in_year) for more information.
   */
  uint8_t months_in_year() const;

  /**
   * Returns the number of days in the month represented by this date
   * 
   * See the [Rust documentation for `days_in_month`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_month) for more information.
   */
  uint8_t days_in_month() const;

  /**
   * Returns the number of days in the year represented by this date
   * 
   * See the [Rust documentation for `days_in_year`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_year) for more information.
   */
  uint32_t days_in_year() const;
  inline const capi::ICU4XIsoDateTime* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XIsoDateTime* AsFFIMut() { return this->inner.get(); }
  inline ICU4XIsoDateTime(capi::ICU4XIsoDateTime* i) : inner(i) {}
  ICU4XIsoDateTime() = default;
  ICU4XIsoDateTime(ICU4XIsoDateTime&&) noexcept = default;
  ICU4XIsoDateTime& operator=(ICU4XIsoDateTime&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XIsoDateTime, ICU4XIsoDateTimeDeleter> inner;
};

#include "ICU4XIsoDate.hpp"
#include "ICU4XTime.hpp"
#include "ICU4XDateTime.hpp"
#include "ICU4XCalendar.hpp"

inline diplomat::result<ICU4XIsoDateTime, ICU4XError> ICU4XIsoDateTime::try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto diplomat_result_raw_out_value = capi::ICU4XIsoDateTime_try_new(year, month, day, hour, minute, second, nanosecond);
  diplomat::result<ICU4XIsoDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XIsoDateTime>(std::move(ICU4XIsoDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XIsoDateTime ICU4XIsoDateTime::new_from_date_and_time(const ICU4XIsoDate& date, const ICU4XTime& time) {
  return ICU4XIsoDateTime(capi::ICU4XIsoDateTime_new_from_date_and_time(date.AsFFI(), time.AsFFI()));
}
inline diplomat::result<ICU4XIsoDateTime, ICU4XError> ICU4XIsoDateTime::from_minutes_since_local_unix_epoch(int32_t minutes) {
  auto diplomat_result_raw_out_value = capi::ICU4XIsoDateTime_from_minutes_since_local_unix_epoch(minutes);
  diplomat::result<ICU4XIsoDateTime, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XIsoDateTime>(std::move(ICU4XIsoDateTime(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XIsoDate ICU4XIsoDateTime::date() const {
  return ICU4XIsoDate(capi::ICU4XIsoDateTime_date(this->inner.get()));
}
inline ICU4XTime ICU4XIsoDateTime::time() const {
  return ICU4XTime(capi::ICU4XIsoDateTime_time(this->inner.get()));
}
inline ICU4XDateTime ICU4XIsoDateTime::to_any() const {
  return ICU4XDateTime(capi::ICU4XIsoDateTime_to_any(this->inner.get()));
}
inline int32_t ICU4XIsoDateTime::minutes_since_local_unix_epoch() const {
  return capi::ICU4XIsoDateTime_minutes_since_local_unix_epoch(this->inner.get());
}
inline ICU4XDateTime ICU4XIsoDateTime::to_calendar(const ICU4XCalendar& calendar) const {
  return ICU4XDateTime(capi::ICU4XIsoDateTime_to_calendar(this->inner.get(), calendar.AsFFI()));
}
inline uint8_t ICU4XIsoDateTime::hour() const {
  return capi::ICU4XIsoDateTime_hour(this->inner.get());
}
inline uint8_t ICU4XIsoDateTime::minute() const {
  return capi::ICU4XIsoDateTime_minute(this->inner.get());
}
inline uint8_t ICU4XIsoDateTime::second() const {
  return capi::ICU4XIsoDateTime_second(this->inner.get());
}
inline uint32_t ICU4XIsoDateTime::nanosecond() const {
  return capi::ICU4XIsoDateTime_nanosecond(this->inner.get());
}
inline uint32_t ICU4XIsoDateTime::day_of_month() const {
  return capi::ICU4XIsoDateTime_day_of_month(this->inner.get());
}
inline ICU4XIsoWeekday ICU4XIsoDateTime::day_of_week() const {
  return static_cast<ICU4XIsoWeekday>(capi::ICU4XIsoDateTime_day_of_week(this->inner.get()));
}
inline uint32_t ICU4XIsoDateTime::month() const {
  return capi::ICU4XIsoDateTime_month(this->inner.get());
}
inline int32_t ICU4XIsoDateTime::year() const {
  return capi::ICU4XIsoDateTime_year(this->inner.get());
}
inline uint8_t ICU4XIsoDateTime::months_in_year() const {
  return capi::ICU4XIsoDateTime_months_in_year(this->inner.get());
}
inline uint8_t ICU4XIsoDateTime::days_in_month() const {
  return capi::ICU4XIsoDateTime_days_in_month(this->inner.get());
}
inline uint32_t ICU4XIsoDateTime::days_in_year() const {
  return capi::ICU4XIsoDateTime_days_in_year(this->inner.get());
}
#endif
