# Username

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**global_access_level** | **String** | The global-scope access-level of a CLI username. The default value is &#x60;\&quot;read-only\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - User has no access to global data. \&quot;read-only\&quot; - User has read-only access to global data. \&quot;read-write\&quot; - User has read-write access to most global data. \&quot;admin\&quot; - User has read-write access to all global data. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_default_access_level** | **String** | The vpn-scope access-level that gets assigned by default to CLI users on each Message VPN unless there is an access-level exception configured for it. In that case the exception takes precedence. The default value is &#x60;\&quot;none\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;none\&quot; - User has no access to a Message VPN. \&quot;read-only\&quot; - User has read-only access to a Message VPN. \&quot;read-write\&quot; - User has read-write access to most Message VPN settings. &lt;/pre&gt;  | [optional] [default to null]
**password** | **String** | Change the password of the user. The default is to have no &#x60;password&#x60;. | [optional] [default to null]
**user_name** | **String** | Username. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


