# {{classname}}

All URIs are relative to */*

| Method                                                                               | HTTP request                                                      | Description            |
| ------------------------------------------------------------------------------------ | ----------------------------------------------------------------- | ---------------------- |
| [**get_detailed_invoice_using_get1**](BillingApi.md#get_detailed_invoice_using_get1) | **GET** users-web/api/v3/billing/invoice/{service}/{year}/{month} | Get invoice details    |
| [**list_available_plans_using_get1**](BillingApi.md#list_available_plans_using_get1) | **GET** users-web/api/v3/billing/availablePlans                   | Get available plans    |
| [**update_plan_using_put1**](BillingApi.md#update_plan_using_put1)                   | **PUT** users-web/api/v3/billing/info/{appId}                     | Update plan for an app |

# **get_detailed_invoice_using_get1**

> InvoiceResponse get_detailed_invoice_using_get1(ctx, service, year, month)
Get invoice details

### Required Parameters

| Name        | Type                | Description                           | Notes                    |
| ----------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**     | **context.Context** | context containing the authentication | nil if no authentication |
| **service** | **String**          | service                               |
| **year**    | **i32**             | year                                  |
| **month**   | **i32**             | month                                 |

### Return type

[**InvoiceResponse**](InvoiceResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_available_plans_using_get1**

> PlansResponse list_available_plans_using_get1(ctx, optional)
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

[**PlansResponse**](PlansResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_plan_using_put1**

> UpdatePlanResponse update_plan_using_put1(ctx, body, app_id)
Update plan for an app

### Required Parameters

| Name       | Type                              | Description                           | Notes                    |
| ---------- | --------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**               | context containing the authentication | nil if no authentication |
| **body**   | [**BillingInfo**](BillingInfo.md) | dto                                   |
| **app_id** | **i64**                           | appId                                 |

### Return type

[**UpdatePlanResponse**](UpdatePlanResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
