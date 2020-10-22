# \BillingApi

All URIs are relative to *http://localhost*

| Method                                                                               | HTTP request                                                       | Description            |
| ------------------------------------------------------------------------------------ | ------------------------------------------------------------------ | ---------------------- |
| [**get_detailed_invoice_using_get1**](BillingApi.md#get_detailed_invoice_using_get1) | **Get** /users-web/api/v3/billing/invoice/{service}/{year}/{month} | Get invoice details    |
| [**list_available_plans_using_get1**](BillingApi.md#list_available_plans_using_get1) | **Get** /users-web/api/v3/billing/availablePlans                   | Get available plans    |
| [**update_plan_using_put**](BillingApi.md#update_plan_using_put)                     | **Put** /users-web/api/v3/billing/info/{appId}                     | Update plan for an app |



## get_detailed_invoice_using_get1

> crate::models::GenericApiResponse get_detailed_invoice_using_get1(service, year, month)
Get invoice details

### Parameters


| Name        | Type       | Description | Required   | Notes |
| ----------- | ---------- | ----------- | ---------- | ----- |
| **service** | **String** | service     | [required] |
| **year**    | **i32**    | year        | [required] |
| **month**   | **i32**    | month       | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_plans_using_get1

> crate::models::GenericApiResponse list_available_plans_using_get1(integration_id, app_type)
Get available plans

### Parameters


| Name               | Type               | Description   | Required | Notes |
| ------------------ | ------------------ | ------------- | -------- | ----- |
| **integration_id** | Option<**i64**>    | integrationId |          |
| **app_type**       | Option<**String**> | appType       |          |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plan_using_put

> crate::models::GenericApiResponse update_plan_using_put(app_id, dto)
Update plan for an app

### Parameters


| Name       | Type                              | Description | Required   | Notes |
| ---------- | --------------------------------- | ----------- | ---------- | ----- |
| **app_id** | **i64**                           | appId       | [required] |
| **dto**    | [**BillingInfo**](BillingInfo.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
