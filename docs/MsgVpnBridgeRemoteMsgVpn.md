# MsgVpnBridgeRemoteMsgVpn

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge_name** | **String** | The name of the Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | The virtual router of the Bridge. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Bridge is used for the primary virtual router. \&quot;backup\&quot; - The Bridge is used for the backup virtual router. \&quot;auto\&quot; - The Bridge is automatically assigned a virtual router at creation, depending on the broker&#39;s active-standby role. &lt;/pre&gt;  | [optional] [default to null]
**client_username** | **String** | The Client Username the Bridge uses to login to the remote Message VPN. This per remote Message VPN value overrides the value provided for the Bridge overall. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**compressed_data_enabled** | **bool** | Enable or disable data compression for the remote Message VPN connection. The default value is &#x60;false&#x60;. | [optional] [default to null]
**connect_order** | **i32** | The preference given to incoming connections from remote Message VPN hosts, from 1 (highest priority) to 4 (lowest priority). The default value is &#x60;4&#x60;. | [optional] [default to null]
**egress_flow_window_size** | **i64** | The number of outstanding guaranteed messages that can be transmitted over the remote Message VPN connection before an acknowledgement is received. The default value is &#x60;255&#x60;. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the remote Message VPN. The default value is &#x60;false&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**password** | **String** | The password for the Client Username. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**queue_binding** | **String** | The queue binding of the Bridge in the remote Message VPN. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**remote_msg_vpn_interface** | **String** | The physical interface on the local Message VPN host for connecting to the remote Message VPN. By default, an interface is chosen automatically (recommended), but if specified, &#x60;remoteMsgVpnLocation&#x60; must not be a virtual router name. | [optional] [default to null]
**remote_msg_vpn_location** | **String** | The location of the remote Message VPN as either an FQDN with port, IP address with port, or virtual router name (starting with \&quot;v:\&quot;). | [optional] [default to null]
**remote_msg_vpn_name** | **String** | The name of the remote Message VPN. | [optional] [default to null]
**tls_enabled** | **bool** | Enable or disable encryption (TLS) for the remote Message VPN connection. The default value is &#x60;false&#x60;. | [optional] [default to null]
**unidirectional_client_profile** | **String** | The Client Profile for the unidirectional Bridge of the remote Message VPN. The Client Profile must exist in the local Message VPN, and it is used only for the TCP parameters. Note that the default client profile has a TCP maximum window size of 2MB. The default value is &#x60;\&quot;#client-profile\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


