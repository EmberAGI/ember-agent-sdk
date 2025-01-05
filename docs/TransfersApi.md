# \TransfersApi

All URIs are relative to *https://api.emberai.xyz/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transfer_post**](TransfersApi.md#create_transfer_post) | **POST** /create_transfer | Creates an unsigned transaction payload for sending tokens.



## create_transfer_post

> models::CreateTransferResponse create_transfer_post(create_transfer_request)
Creates an unsigned transaction payload for sending tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_transfer_request** | [**CreateTransferRequest**](CreateTransferRequest.md) | Transfer request payload. | [required] |

### Return type

[**models::CreateTransferResponse**](CreateTransferResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

