# SempMeta

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **i64** | The total number of objects requested, irrespective of page size. This may be a count of all objects in a collection or a filtered subset. It represents a snapshot in time and may change when paging through results. | [optional] [default to null]
**error** | [***::models::SempError**](SempError.md) |  | [optional] [default to null]
**paging** | [***::models::SempPaging**](SempPaging.md) |  | [optional] [default to null]
**request** | [***::models::SempRequest**](SempRequest.md) |  | [default to null]
**response_code** | **i32** | The HTTP response code, one of 200 (success), 4xx (client error), or 5xx (server error). | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


