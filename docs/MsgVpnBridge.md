# MsgVpnBridge

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge_name** | **String** | The name of the Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | The virtual router of the Bridge. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Bridge is used for the primary virtual router. \&quot;backup\&quot; - The Bridge is used for the backup virtual router. \&quot;auto\&quot; - The Bridge is automatically assigned a router. &lt;/pre&gt;  | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Bridge. The default value is &#x60;false&#x60;. | [optional] [default to null]
**max_ttl** | **i64** | The maximum time-to-live (TTL) in hops. Messages are discarded if their TTL exceeds this value. The default value is &#x60;8&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**remote_authentication_basic_client_username** | **String** | The Client Username the Bridge uses to login to the remote Message VPN. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**remote_authentication_basic_password** | **String** | The password for the Client Username. The default is to have no &#x60;remoteAuthenticationBasicPassword&#x60;. | [optional] [default to null]
**remote_authentication_client_cert_content** | **String** | The PEM formatted content for the client certificate used by the Bridge to login to the remote Message VPN. It must consist of a private key and between one and three certificates comprising the certificate trust chain. The default value is &#x60;\&quot;\&quot;&#x60;. Available since 2.9. | [optional] [default to null]
**remote_authentication_client_cert_password** | **String** | The password for the client certificate. The default value is &#x60;\&quot;\&quot;&#x60;. Available since 2.9. | [optional] [default to null]
**remote_authentication_scheme** | **String** | The authentication scheme for the remote Message VPN. The default value is &#x60;\&quot;basic\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;basic\&quot; - Basic Authentication Scheme (via username and password). \&quot;client-certificate\&quot; - Client Certificate Authentication Scheme (via certificate file or content). &lt;/pre&gt;  | [optional] [default to null]
**remote_connection_retry_count** | **i64** | The maximum number of retry attempts to establish a connection to the remote Message VPN. A value of 0 means to retry forever. The default value is &#x60;0&#x60;. | [optional] [default to null]
**remote_connection_retry_delay** | **i64** | The number of seconds to delay before retrying to connect to the remote Message VPN. The default value is &#x60;3&#x60;. | [optional] [default to null]
**remote_deliver_to_one_priority** | **String** | The priority for deliver-to-one (DTO) messages transmitted from the remote Message VPN. The default value is &#x60;\&quot;p1\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;p1\&quot; - Priority 1 (highest). \&quot;p2\&quot; - Priority 2. \&quot;p3\&quot; - Priority 3. \&quot;p4\&quot; - Priority 4 (lowest). \&quot;da\&quot; - Deliver Always. &lt;/pre&gt;  | [optional] [default to null]
**tls_cipher_suite_list** | **String** | The colon-separated list of cipher-suites supported for TLS connections to the remote Message VPN. The value \&quot;default\&quot; implies all supported suites ordered from most secure to least secure. The default value is &#x60;\&quot;default\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


