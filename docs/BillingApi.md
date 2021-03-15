# \BillingApi

All URIs are relative to *https://localhost*

| Method                                                                             | HTTP request                                                       | Description            |
| ---------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ---------------------- |
| [**get_detailed_invoice_using_get**](BillingApi.md#get_detailed_invoice_using_get) | **Get** /users-web/api/v3/billing/invoice/{service}/{year}/{month} | Get invoice details    |
| [**list_available_plans_using_get**](BillingApi.md#list_available_plans_using_get) | **Get** /users-web/api/v3/billing/availablePlans                   | Get available plans    |
| [**update_plan_using_put**](BillingApi.md#update_plan_using_put)                   | **Put** /users-web/api/v3/billing/info/{appId}                     | Update plan for an app |


# **get_detailed_invoice_using_get**
> ::models::InvoiceResponse get_detailed_invoice_using_get(ctx, service, year, month)
Get invoice details

### Required Parameters

| Name        | Type                | Description                           | Notes                    |
| ----------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**     | **context.Context** | context containing the authentication | nil if no authentication |
| **service** | **String**          | service                               |
| **year**    | **i32**             | year                                  |
| **month**   | **i32**             | month                                 |

### Return type

[**::models::InvoiceResponse**](InvoiceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_available_plans_using_get**
> ::models::PlansResponse list_available_plans_using_get(ctx, optional)
Get available plans

### Required Parameters

| Name         | Type                       | Description                           | Notes                    |
| ------------ | -------------------------- | ------------------------------------- | ------------------------ |
| **ctx**      | **context.Context**        | context containing the authentication | nil if no authentication |
| **optional** | **map[string]interface{}** | optional parameters                   | nil if no parameters     |

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

| Name               | Type       | Description   | Notes |
| ------------------ | ---------- | ------------- | ----- |
| **integration_id** | **i64**    | integrationId |
| **app_type**       | **String** | appType       |

### Return type

[**::models::PlansResponse**](PlansResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_plan_using_put**
> ::models::UpdatePlanResponse update_plan_using_put(ctx, app_id, dto)
Update plan for an app

### Required Parameters

| Name       | Type                              | Description                           | Notes                    |
| ---------- | --------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**               | context containing the authentication | nil if no authentication |
| **app_id** | **i64**                           | appId                                 |
| **dto**    | [**BillingInfo**](BillingInfo.md) | dto                                   |

### Return type

[**::models::UpdatePlanResponse**](UpdatePlanResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
