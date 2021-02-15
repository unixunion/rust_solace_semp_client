# MsgVpnReplicatedTopic

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**replicated_topic** | **String** | The topic for applying replication. Published messages matching this topic will be replicated to the standby site. | [optional] [default to null]
**replication_mode** | **String** | The replication mode for the Replicated Topic. The default value is &#x60;\&quot;async\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;sync\&quot; - Messages are acknowledged when replicated (spooled remotely). \&quot;async\&quot; - Messages are acknowledged when pending replication (spooled locally). &lt;/pre&gt;  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


