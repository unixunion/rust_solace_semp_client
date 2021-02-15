# MsgVpnAuthorizationGroup

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl_profile_name** | **String** | The ACL Profile of the LDAP Authorization Group. The default value is &#x60;\&quot;default\&quot;&#x60;. | [optional] [default to null]
**authorization_group_name** | **String** | The name of the LDAP Authorization Group. Special care is needed if the group name contains special characters such as &#39;#&#39;, &#39;+&#39;, &#39;;&#39;, &#39;&#x3D;&#39; as the value of the group name returned from the LDAP server might prepend those characters with &#39;\\&#39;. For example a group name called &#39;test#,lab,com&#39; will be returned from the LDAP server as &#39;test\\#,lab,com&#39;. | [optional] [default to null]
**client_profile_name** | **String** | The Client Profile of the LDAP Authorization Group. The default value is &#x60;\&quot;default\&quot;&#x60;. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the LDAP Authorization Group in the Message VPN. The default value is &#x60;false&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**order_after_authorization_group_name** | **String** | Lower the priority to be less than this group. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. The default is not applicable. | [optional] [default to null]
**order_before_authorization_group_name** | **String** | Raise the priority to be greater than this group. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. The default is not applicable. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


