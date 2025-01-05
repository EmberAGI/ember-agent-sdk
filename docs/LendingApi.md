# \LendingApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lend_tokens_post**](LendingApi.md#lend_tokens_post) | **POST** /lend_tokens | Supplies assets to a lending protocol.
[**withdraw_lent_tokens_post**](LendingApi.md#withdraw_lent_tokens_post) | **POST** /withdraw_lent_tokens | Withdraws assets from a lending position.



## lend_tokens_post

> models::LendTokensResponse lend_tokens_post(lend_tokens_request)
Supplies assets to a lending protocol.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_tokens_request** | [**LendTokensRequest**](LendTokensRequest.md) | Lend tokens request payload. | [required] |

### Return type

[**models::LendTokensResponse**](LendTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_lent_tokens_post

> models::WithdrawLentTokensResponse withdraw_lent_tokens_post(withdraw_lent_tokens_request)
Withdraws assets from a lending position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**withdraw_lent_tokens_request** | [**WithdrawLentTokensRequest**](WithdrawLentTokensRequest.md) | Withdraw lent tokens request payload. | [required] |

### Return type

[**models::WithdrawLentTokensResponse**](WithdrawLentTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

