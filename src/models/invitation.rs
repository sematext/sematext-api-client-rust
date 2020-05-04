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
pub struct Invitation {
  /// For invite request, only app.id needs to be set.
  #[serde(rename = "app")]
  app: Option<::models::App>,
  /// For invite request, only apps.id needs to be set.
  #[serde(rename = "apps")]
  apps: Option<Vec<::models::App>>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "inviteDate")]
  invite_date: Option<String>,
  #[serde(rename = "inviteStatus")]
  invite_status: Option<String>,
  #[serde(rename = "inviteeEmail")]
  invitee_email: Option<String>,
  #[serde(rename = "inviteeRole")]
  invitee_role: Option<String>,
  #[serde(rename = "inviteeStatus")]
  invitee_status: Option<String>,
  #[serde(rename = "inviterEmail")]
  inviter_email: Option<String>,
  #[serde(rename = "uuid")]
  uuid: Option<String>
}

impl Invitation {
  pub fn new() -> Invitation {
    Invitation {
      app: None,
      apps: None,
      id: None,
      invite_date: None,
      invite_status: None,
      invitee_email: None,
      invitee_role: None,
      invitee_status: None,
      inviter_email: None,
      uuid: None
    }
  }

  pub fn set_app(&mut self, app: ::models::App) {
    self.app = Some(app);
  }

  pub fn with_app(mut self, app: ::models::App) -> Invitation {
    self.app = Some(app);
    self
  }

  pub fn app(&self) -> Option<&::models::App> {
    self.app.as_ref()
  }

  pub fn reset_app(&mut self) {
    self.app = None;
  }

  pub fn set_apps(&mut self, apps: Vec<::models::App>) {
    self.apps = Some(apps);
  }

  pub fn with_apps(mut self, apps: Vec<::models::App>) -> Invitation {
    self.apps = Some(apps);
    self
  }

  pub fn apps(&self) -> Option<&Vec<::models::App>> {
    self.apps.as_ref()
  }

  pub fn reset_apps(&mut self) {
    self.apps = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> Invitation {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_invite_date(&mut self, invite_date: String) {
    self.invite_date = Some(invite_date);
  }

  pub fn with_invite_date(mut self, invite_date: String) -> Invitation {
    self.invite_date = Some(invite_date);
    self
  }

  pub fn invite_date(&self) -> Option<&String> {
    self.invite_date.as_ref()
  }

  pub fn reset_invite_date(&mut self) {
    self.invite_date = None;
  }

  pub fn set_invite_status(&mut self, invite_status: String) {
    self.invite_status = Some(invite_status);
  }

  pub fn with_invite_status(mut self, invite_status: String) -> Invitation {
    self.invite_status = Some(invite_status);
    self
  }

  pub fn invite_status(&self) -> Option<&String> {
    self.invite_status.as_ref()
  }

  pub fn reset_invite_status(&mut self) {
    self.invite_status = None;
  }

  pub fn set_invitee_email(&mut self, invitee_email: String) {
    self.invitee_email = Some(invitee_email);
  }

  pub fn with_invitee_email(mut self, invitee_email: String) -> Invitation {
    self.invitee_email = Some(invitee_email);
    self
  }

  pub fn invitee_email(&self) -> Option<&String> {
    self.invitee_email.as_ref()
  }

  pub fn reset_invitee_email(&mut self) {
    self.invitee_email = None;
  }

  pub fn set_invitee_role(&mut self, invitee_role: String) {
    self.invitee_role = Some(invitee_role);
  }

  pub fn with_invitee_role(mut self, invitee_role: String) -> Invitation {
    self.invitee_role = Some(invitee_role);
    self
  }

  pub fn invitee_role(&self) -> Option<&String> {
    self.invitee_role.as_ref()
  }

  pub fn reset_invitee_role(&mut self) {
    self.invitee_role = None;
  }

  pub fn set_invitee_status(&mut self, invitee_status: String) {
    self.invitee_status = Some(invitee_status);
  }

  pub fn with_invitee_status(mut self, invitee_status: String) -> Invitation {
    self.invitee_status = Some(invitee_status);
    self
  }

  pub fn invitee_status(&self) -> Option<&String> {
    self.invitee_status.as_ref()
  }

  pub fn reset_invitee_status(&mut self) {
    self.invitee_status = None;
  }

  pub fn set_inviter_email(&mut self, inviter_email: String) {
    self.inviter_email = Some(inviter_email);
  }

  pub fn with_inviter_email(mut self, inviter_email: String) -> Invitation {
    self.inviter_email = Some(inviter_email);
    self
  }

  pub fn inviter_email(&self) -> Option<&String> {
    self.inviter_email.as_ref()
  }

  pub fn reset_inviter_email(&mut self) {
    self.inviter_email = None;
  }

  pub fn set_uuid(&mut self, uuid: String) {
    self.uuid = Some(uuid);
  }

  pub fn with_uuid(mut self, uuid: String) -> Invitation {
    self.uuid = Some(uuid);
    self
  }

  pub fn uuid(&self) -> Option<&String> {
    self.uuid.as_ref()
  }

  pub fn reset_uuid(&mut self) {
    self.uuid = None;
  }

}



