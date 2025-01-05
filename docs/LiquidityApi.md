# \LiquidityApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_liquidity_post**](LiquidityApi.md#add_liquidity_post) | **POST** /add_liquidity | Creates a new liquidity position in a DEX pool.
[**remove_liquidity_post**](LiquidityApi.md#remove_liquidity_post) | **POST** /remove_liquidity | Removes liquidity from a DEX pool position.



## add_liquidity_post

> models::AddLiquidityResponse add_liquidity_post(add_liquidity_request)
Creates a new liquidity position in a DEX pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_liquidity_request** | [**AddLiquidityRequest**](AddLiquidityRequest.md) | Add liquidity request payload. | [required] |

### Return type

[**models::AddLiquidityResponse**](AddLiquidityResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_liquidity_post

> models::RemoveLiquidityResponse remove_liquidity_post(remove_liquidity_request)
Removes liquidity from a DEX pool position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_liquidity_request** | [**RemoveLiquidityRequest**](RemoveLiquidityRequest.md) | Remove liquidity request payload. | [required] |

### Return type

[**models::RemoveLiquidityResponse**](RemoveLiquidityResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

