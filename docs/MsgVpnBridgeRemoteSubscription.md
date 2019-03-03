# MsgVpnBridgeRemoteSubscription

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge_name** | **String** | The name of the Message VPN Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | Specify whether the Message VPN Bridge is used for the Primary or Backup Router. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Message VPN Bridge is used for the Primary Router. \&quot;backup\&quot; - The Message VPN Bridge is used for the Backup Router. &lt;/pre&gt;  | [optional] [default to null]
**deliver_always_enabled** | **bool** | Flag the topic as deliver-always instead of with the configured deliver-to-one remote-priority value for the bridge. A given topic may be deliver-to-one or deliver-always but not both. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**remote_subscription_topic** | **String** | The Topic of the Remote Subscription. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


