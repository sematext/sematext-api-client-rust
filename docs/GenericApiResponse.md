# GenericApiResponse

## Properties

| Name        | Type                                              | Description                                                                                  | Notes      |
| ----------- | ------------------------------------------------- | -------------------------------------------------------------------------------------------- | ---------- |
| **data**    | Option<[**serde_json::Value**](.md)>              | Contains actual data when response is successful. Key and Value is specific to each endpoint | [optional] |
| **errors**  | Option<[**Vec<crate::models::Error>**](Error.md)> |                                                                                              | [optional] |
| **message** | Option<**String**>                                |                                                                                              | [optional] |
| **success** | Option<**bool**>                                  |                                                                                              | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
