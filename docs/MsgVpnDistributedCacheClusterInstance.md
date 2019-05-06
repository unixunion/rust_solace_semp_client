# MsgVpnDistributedCacheClusterInstance

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_start_enabled** | **bool** | Enable or disable auto-start for the Cache Instance. When enabled, the Cache Instance will automatically attempt to transition from the Stopped operational state to Up whenever it restarts or reconnects to the message broker. The default value is &#x60;false&#x60;. | [optional] [default to null]
**cache_name** | **String** | The name of the Distributed Cache. | [optional] [default to null]
**cluster_name** | **String** | The name of the Cache Cluster. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Cache Instance. The default value is &#x60;false&#x60;. | [optional] [default to null]
**instance_name** | **String** | The name of the Cache Instance. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**stop_on_lost_msg_enabled** | **bool** | Enable or disable stop-on-lost-message for the Cache Instance. When enabled, the Cache Instance will transition to the stopped operational state upon losing a message. When stopped, it cannot accept or respond to cache requests, but continues to cache messages. The default value is &#x60;true&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


