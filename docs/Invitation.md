# Invitation

## Properties

| Name               | Type                                          | Description                                       | Notes                |
| ------------------ | --------------------------------------------- | ------------------------------------------------- | -------------------- |
| **app**            | Option<[**crate::models::App**](App.md)>      |                                                   | [optional]           |
| **apps**           | Option<[**Vec<crate::models::App>**](App.md)> | For invite request, only apps.id needs to be set. | [optional]           |
| **id**             | Option<**i64**>                               |                                                   | [optional][readonly] |
| **invite_date**    | Option<**String**>                            |                                                   | [optional][readonly] |
| **invite_status**  | Option<**String**>                            |                                                   | [optional][readonly] |
| **invitee_email**  | Option<**String**>                            |                                                   | [optional]           |
| **invitee_role**   | Option<**String**>                            |                                                   | [optional]           |
| **invitee_status** | Option<**String**>                            |                                                   | [optional][readonly] |
| **inviter_email**  | Option<**String**>                            |                                                   | [optional][readonly] |
| **uuid**           | Option<**String**>                            |                                                   | [optional][readonly] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
