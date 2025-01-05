# \SwapsApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_limit_order_post**](SwapsApi.md#create_limit_order_post) | **POST** /create_limit_order | Creates a limit order for token swaps.
[**create_swap_post**](SwapsApi.md#create_swap_post) | **POST** /create_swap | Creates an unsigned transaction payload for swapping tokens.



## create_limit_order_post

> models::CreateLimitOrderResponse create_limit_order_post(create_limit_order_request)
Creates a limit order for token swaps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_limit_order_request** | [**CreateLimitOrderRequest**](CreateLimitOrderRequest.md) | Limit order request payload. | [required] |

### Return type

[**models::CreateLimitOrderResponse**](CreateLimitOrderResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_swap_post

> models::CreateSwapResponse create_swap_post(create_swap_request)
Creates an unsigned transaction payload for swapping tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_swap_request** | [**CreateSwapRequest**](CreateSwapRequest.md) | Swap request payload. | [required] |

### Return type

[**models::CreateSwapResponse**](CreateSwapResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

