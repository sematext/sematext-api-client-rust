/* 
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * OpenAPI spec version: v3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageDto {
  #[serde(rename = "count")]
  count: Option<i64>,
  #[serde(rename = "dailyUsage")]
  daily_usage: Option<Vec<::models::DailyDto>>,
  #[serde(rename = "dailyVolumeMB")]
  daily_volume_mb: Option<i64>,
  #[serde(rename = "end")]
  end: Option<String>,
  #[serde(rename = "failedCount")]
  failed_count: Option<i64>,
  #[serde(rename = "limitChangeEvents")]
  limit_change_events: Option<Vec<::models::LimitChangeEventDto>>,
  #[serde(rename = "maxAllowedMB")]
  max_allowed_mb: Option<i64>,
  #[serde(rename = "maxLimitMB")]
  max_limit_mb: Option<i64>,
  #[serde(rename = "start")]
  start: Option<String>,
  #[serde(rename = "volume")]
  volume: Option<i64>,
  #[serde(rename = "volumeChangeEvents")]
  volume_change_events: Option<Vec<::models::LimitChangeEventDto>>
}

impl UsageDto {
  pub fn new() -> UsageDto {
    UsageDto {
      count: None,
      daily_usage: None,
      daily_volume_mb: None,
      end: None,
      failed_count: None,
      limit_change_events: None,
      max_allowed_mb: None,
      max_limit_mb: None,
      start: None,
      volume: None,
      volume_change_events: None
    }
  }

  pub fn set_count(&mut self, count: i64) {
    self.count = Some(count);
  }

  pub fn with_count(mut self, count: i64) -> UsageDto {
    self.count = Some(count);
    self
  }

  pub fn count(&self) -> Option<&i64> {
    self.count.as_ref()
  }

  pub fn reset_count(&mut self) {
    self.count = None;
  }

  pub fn set_daily_usage(&mut self, daily_usage: Vec<::models::DailyDto>) {
    self.daily_usage = Some(daily_usage);
  }

  pub fn with_daily_usage(mut self, daily_usage: Vec<::models::DailyDto>) -> UsageDto {
    self.daily_usage = Some(daily_usage);
    self
  }

  pub fn daily_usage(&self) -> Option<&Vec<::models::DailyDto>> {
    self.daily_usage.as_ref()
  }

  pub fn reset_daily_usage(&mut self) {
    self.daily_usage = None;
  }

  pub fn set_daily_volume_mb(&mut self, daily_volume_mb: i64) {
    self.daily_volume_mb = Some(daily_volume_mb);
  }

  pub fn with_daily_volume_mb(mut self, daily_volume_mb: i64) -> UsageDto {
    self.daily_volume_mb = Some(daily_volume_mb);
    self
  }

  pub fn daily_volume_mb(&self) -> Option<&i64> {
    self.daily_volume_mb.as_ref()
  }

  pub fn reset_daily_volume_mb(&mut self) {
    self.daily_volume_mb = None;
  }

  pub fn set_end(&mut self, end: String) {
    self.end = Some(end);
  }

  pub fn with_end(mut self, end: String) -> UsageDto {
    self.end = Some(end);
    self
  }

  pub fn end(&self) -> Option<&String> {
    self.end.as_ref()
  }

  pub fn reset_end(&mut self) {
    self.end = None;
  }

  pub fn set_failed_count(&mut self, failed_count: i64) {
    self.failed_count = Some(failed_count);
  }

  pub fn with_failed_count(mut self, failed_count: i64) -> UsageDto {
    self.failed_count = Some(failed_count);
    self
  }

  pub fn failed_count(&self) -> Option<&i64> {
    self.failed_count.as_ref()
  }

  pub fn reset_failed_count(&mut self) {
    self.failed_count = None;
  }

  pub fn set_limit_change_events(&mut self, limit_change_events: Vec<::models::LimitChangeEventDto>) {
    self.limit_change_events = Some(limit_change_events);
  }

  pub fn with_limit_change_events(mut self, limit_change_events: Vec<::models::LimitChangeEventDto>) -> UsageDto {
    self.limit_change_events = Some(limit_change_events);
    self
  }

  pub fn limit_change_events(&self) -> Option<&Vec<::models::LimitChangeEventDto>> {
    self.limit_change_events.as_ref()
  }

  pub fn reset_limit_change_events(&mut self) {
    self.limit_change_events = None;
  }

  pub fn set_max_allowed_mb(&mut self, max_allowed_mb: i64) {
    self.max_allowed_mb = Some(max_allowed_mb);
  }

  pub fn with_max_allowed_mb(mut self, max_allowed_mb: i64) -> UsageDto {
    self.max_allowed_mb = Some(max_allowed_mb);
    self
  }

  pub fn max_allowed_mb(&self) -> Option<&i64> {
    self.max_allowed_mb.as_ref()
  }

  pub fn reset_max_allowed_mb(&mut self) {
    self.max_allowed_mb = None;
  }

  pub fn set_max_limit_mb(&mut self, max_limit_mb: i64) {
    self.max_limit_mb = Some(max_limit_mb);
  }

  pub fn with_max_limit_mb(mut self, max_limit_mb: i64) -> UsageDto {
    self.max_limit_mb = Some(max_limit_mb);
    self
  }

  pub fn max_limit_mb(&self) -> Option<&i64> {
    self.max_limit_mb.as_ref()
  }

  pub fn reset_max_limit_mb(&mut self) {
    self.max_limit_mb = None;
  }

  pub fn set_start(&mut self, start: String) {
    self.start = Some(start);
  }

  pub fn with_start(mut self, start: String) -> UsageDto {
    self.start = Some(start);
    self
  }

  pub fn start(&self) -> Option<&String> {
    self.start.as_ref()
  }

  pub fn reset_start(&mut self) {
    self.start = None;
  }

  pub fn set_volume(&mut self, volume: i64) {
    self.volume = Some(volume);
  }

  pub fn with_volume(mut self, volume: i64) -> UsageDto {
    self.volume = Some(volume);
    self
  }

  pub fn volume(&self) -> Option<&i64> {
    self.volume.as_ref()
  }

  pub fn reset_volume(&mut self) {
    self.volume = None;
  }

  pub fn set_volume_change_events(&mut self, volume_change_events: Vec<::models::LimitChangeEventDto>) {
    self.volume_change_events = Some(volume_change_events);
  }

  pub fn with_volume_change_events(mut self, volume_change_events: Vec<::models::LimitChangeEventDto>) -> UsageDto {
    self.volume_change_events = Some(volume_change_events);
    self
  }

  pub fn volume_change_events(&self) -> Option<&Vec<::models::LimitChangeEventDto>> {
    self.volume_change_events.as_ref()
  }

  pub fn reset_volume_change_events(&mut self) {
    self.volume_change_events = None;
  }

}



