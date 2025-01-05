# \TransactionsApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**submit_transaction_post**](TransactionsApi.md#submit_transaction_post) | **POST** /submit_transaction | Submits a signed transaction to the network.



## submit_transaction_post

> models::SubmitTransactionResponse submit_transaction_post(submit_transaction_request)
Submits a signed transaction to the network.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_transaction_request** | [**SubmitTransactionRequest**](SubmitTransactionRequest.md) | Submit transaction request payload. | [required] |

### Return type

[**models::SubmitTransactionResponse**](SubmitTransactionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

