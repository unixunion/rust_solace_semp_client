# UsernameMsgVpnAccessLevelException

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_level** | **String** | vpn-scope access-level to assign to CLI users. The default value is &#x60;\&quot;none\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - User has no access to a Message VPN. \&quot;read-only\&quot; - User has read-only access to a Message VPN. \&quot;read-write\&quot; - User has read-write access to most Message VPN settings. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN for which an access level exception may be configured. | [optional] [default to null]
**user_name** | **String** | Username. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


