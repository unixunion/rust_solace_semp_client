# MsgVpnDistributedCache

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_name** | **String** | The name of the Distributed Cache. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Distributed Cache. The default value is &#x60;false&#x60;. | [optional] [default to null]
**heartbeat** | **i64** | The heartbeat interval, in seconds, used by the Cache Instances to monitor connectivity with the message broker. The default value is &#x60;10&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**scheduled_delete_msg_day_list** | **String** | The scheduled delete message day(s), specified as \&quot;daily\&quot; or a comma-separated list of days. Days must be specified as \&quot;Sun\&quot;, \&quot;Mon\&quot;, \&quot;Tue\&quot;, \&quot;Wed\&quot;, \&quot;Thu\&quot;, \&quot;Fri\&quot;, or \&quot;Sat\&quot;, with no spaces, and in sorted order from Sunday to Saturday. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**scheduled_delete_msg_time_list** | **String** | The scheduled delete message time(s), specified as \&quot;hourly\&quot; or a comma-separated list of 24-hour times in the form hh:mm, or h:mm. There must be no spaces, and times must be in sorted order from 0:00 to 23:59. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


