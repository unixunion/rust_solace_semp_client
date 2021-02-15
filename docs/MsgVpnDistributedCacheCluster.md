# MsgVpnDistributedCacheCluster

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_name** | **String** | The name of the Distributed Cache. | [optional] [default to null]
**cluster_name** | **String** | The name of the Cache Cluster. | [optional] [default to null]
**deliver_to_one_override_enabled** | **bool** | Enable or disable deliver-to-one override for the Cache Cluster. The default value is &#x60;true&#x60;. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Cache Cluster. The default value is &#x60;false&#x60;. | [optional] [default to null]
**event_data_byte_rate_threshold** | [***::models::EventThresholdByValue**](EventThresholdByValue.md) |  | [optional] [default to null]
**event_data_msg_rate_threshold** | [***::models::EventThresholdByValue**](EventThresholdByValue.md) |  | [optional] [default to null]
**event_max_memory_threshold** | [***::models::EventThresholdByPercent**](EventThresholdByPercent.md) |  | [optional] [default to null]
**event_max_topics_threshold** | [***::models::EventThresholdByPercent**](EventThresholdByPercent.md) |  | [optional] [default to null]
**event_request_queue_depth_threshold** | [***::models::EventThresholdByPercent**](EventThresholdByPercent.md) |  | [optional] [default to null]
**event_request_rate_threshold** | [***::models::EventThresholdByValue**](EventThresholdByValue.md) |  | [optional] [default to null]
**event_response_rate_threshold** | [***::models::EventThresholdByValue**](EventThresholdByValue.md) |  | [optional] [default to null]
**global_caching_enabled** | **bool** | Enable or disable global caching for the Cache Cluster. When enabled, the Cache Instances will fetch topics from remote Home Cache Clusters when requested, and subscribe to those topics to cache them locally. When disabled, the Cache Instances will remove all subscriptions and cached messages for topics from remote Home Cache Clusters. The default value is &#x60;false&#x60;. | [optional] [default to null]
**global_caching_heartbeat** | **i64** | The heartbeat interval, in seconds, used by the Cache Instances to monitor connectivity with the remote Home Cache Clusters. The default value is &#x60;3&#x60;. | [optional] [default to null]
**global_caching_topic_lifetime** | **i64** | The topic lifetime, in seconds. If no client requests are received for a given global topic over the duration of the topic lifetime, then the Cache Instance will remove the subscription and cached messages for that topic. A value of 0 disables aging. The default value is &#x60;3600&#x60;. | [optional] [default to null]
**max_memory** | **i64** | The maximum memory usage, in megabytes (MB), for each Cache Instance in the Cache Cluster. The default value is &#x60;2048&#x60;. | [optional] [default to null]
**max_msgs_per_topic** | **i64** | The maximum number of messages per topic for each Cache Instance in the Cache Cluster. When at the maximum, old messages are removed as new messages arrive. The default value is &#x60;1&#x60;. | [optional] [default to null]
**max_request_queue_depth** | **i64** | The maximum queue depth for cache requests received by the Cache Cluster. The default value is &#x60;100000&#x60;. | [optional] [default to null]
**max_topic_count** | **i64** | The maximum number of topics for each Cache Instance in the Cache Cluster. The default value is &#x60;2000000&#x60;. | [optional] [default to null]
**msg_lifetime** | **i64** | The message lifetime, in seconds. If a message remains cached for the duration of its lifetime, the Cache Instance will remove the message. A lifetime of 0 results in the message being retained indefinitely. The default value is &#x60;0&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**new_topic_advertisement_enabled** | **bool** | Enable or disable the advertising, onto the message bus, of new topics learned by each Cache Instance in the Cache Cluster. The default value is &#x60;false&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


