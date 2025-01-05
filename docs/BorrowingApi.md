# \BorrowingApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**borrow_tokens_post**](BorrowingApi.md#borrow_tokens_post) | **POST** /borrow_tokens | Creates a new borrowing position using deposited collateral.
[**repay_borrowed_tokens_post**](BorrowingApi.md#repay_borrowed_tokens_post) | **POST** /repay_borrowed_tokens | Repays borrowed assets.



## borrow_tokens_post

> models::BorrowTokensResponse borrow_tokens_post(borrow_tokens_request)
Creates a new borrowing position using deposited collateral.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**borrow_tokens_request** | [**BorrowTokensRequest**](BorrowTokensRequest.md) | Borrow tokens request payload. | [required] |

### Return type

[**models::BorrowTokensResponse**](BorrowTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repay_borrowed_tokens_post

> models::RepayBorrowedTokensResponse repay_borrowed_tokens_post(repay_borrowed_tokens_request)
Repays borrowed assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repay_borrowed_tokens_request** | [**RepayBorrowedTokensRequest**](RepayBorrowedTokensRequest.md) | Repay borrowed tokens request payload. | [required] |

### Return type

[**models::RepayBorrowedTokensResponse**](RepayBorrowedTokensResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

