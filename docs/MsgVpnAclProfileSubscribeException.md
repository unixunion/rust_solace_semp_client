# MsgVpnAclProfileSubscribeException

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl_profile_name** | **String** | The name of the ACL Profile. Deprecated since 2.14. Replaced by subscribeTopicExceptions. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. Deprecated since 2.14. Replaced by subscribeTopicExceptions. | [optional] [default to null]
**subscribe_exception_topic** | **String** | The topic for the exception to the default action taken. May include wildcard characters. Deprecated since 2.14. Replaced by subscribeTopicExceptions. | [optional] [default to null]
**topic_syntax** | **String** | The syntax of the topic for the exception to the default action taken. The allowed values and their meaning are:  &lt;pre&gt; \&quot;smf\&quot; - Topic uses SMF syntax. \&quot;mqtt\&quot; - Topic uses MQTT syntax. &lt;/pre&gt;  Deprecated since 2.14. Replaced by subscribeTopicExceptions. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


