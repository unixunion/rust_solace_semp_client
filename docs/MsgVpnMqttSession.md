# MsgVpnMqttSession

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | Enable or disable the MQTT Session. When disabled a client attempting to connect to this session will be denied, and an existing connection will be closed. QoS 1 subscriptions of an MQTT Session will continue to attract messages while disabled. The default value is &#x60;false&#x60;. | [optional] [default to null]
**mqtt_session_client_id** | **String** | The client-id of the MQTT Session, which corresponds to the ClientId provided in the MQTT CONNECT packet. | [optional] [default to null]
**mqtt_session_virtual_router** | **String** | The Virtual Router of the MQTT Session. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - MQTT Session belongs to the Primary Virtual Router. \&quot;backup\&quot; - MQTT Session belongs to the Backup Virtual Router. &lt;/pre&gt;  | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**owner** | **String** | The owner of the MQTT Session. For externally-created sessions this will be the Client Username of the connecting client. For management-created sessions this will be empty by default. In either case the owner can be changed by the administrator. The MQTT Session must be disabled to change its owner. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


