# MsgVpnBridge

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bridge_name** | **String** | The name of the Message VPN Bridge. | [optional] [default to null]
**bridge_virtual_router** | **String** | Specify whether the Message VPN Bridge is used for the Primary or Backup Router. The allowed values and their meaning are:  &lt;pre&gt; \&quot;primary\&quot; - The Message VPN Bridge is used for the Primary Router. \&quot;backup\&quot; - The Message VPN Bridge is used for the Backup Router. &lt;/pre&gt;  | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Message VPN Bridge. The default value is &#x60;false&#x60;. | [optional] [default to null]
**max_ttl** | **i64** | The maximum number of hops (intermediate routers through which data must pass between source and destination) that can occur before the message is discarded. When the Message VPN Bridge sends a message to the remote router, the message TTL value is assigned to the lower of the message current TTL or this value. The default value is &#x60;8&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**remote_authentication_basic_client_username** | **String** | The Client Username the Message VPN Bridge uses to login to the Remote Message VPN. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**remote_authentication_basic_password** | **String** | The password the Message VPN Bridge uses to login to the Remote Message VPN. The default is to have no &#x60;remoteAuthenticationBasicPassword&#x60;. | [optional] [default to null]
**remote_authentication_scheme** | **String** | The Authentication Scheme for the Remote Message VPN. The default value is &#x60;\&quot;basic\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;basic\&quot; - Basic Authentication Scheme (via username and password). \&quot;client-certificate\&quot; - Client Certificate Authentication Scheme (via certificate-file). \&quot;not-applicable\&quot; - Monitor Only Value. &lt;/pre&gt;  | [optional] [default to null]
**remote_connection_retry_count** | **i64** | The maximum number of attempts to establish connection to the Remote Message VPN. The default value is &#x60;0&#x60;. | [optional] [default to null]
**remote_connection_retry_delay** | **i64** | The amount of time before making another attempt to connect to the Remote Message VPN after a failed one, in seconds. The default value is &#x60;3&#x60;. | [optional] [default to null]
**remote_deliver_to_one_priority** | **String** | The priority for deliver-to-one (DTO) messages sent from the Remote Message VPN to the Message VPN Bridge. The default value is &#x60;\&quot;p1\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;p1\&quot; - Priority 1 (highest). \&quot;p2\&quot; - Priority 2. \&quot;p3\&quot; - Priority 3. \&quot;p4\&quot; - Priority 4 (lowest). \&quot;da\&quot; - Deliver Always. &lt;/pre&gt;  | [optional] [default to null]
**tls_cipher_suite_list** | **String** | The list of of cipher suites supported for TLS connections to the Remote Message VPN. The default value is &#x60;\&quot;ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-SHA384:ECDHE-RSA-AES256-SHA:AES256-GCM-SHA384:AES256-SHA256:AES256-SHA:ECDHE-RSA-DES-CBC3-SHA:DES-CBC3-SHA:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-SHA256:ECDHE-RSA-AES128-SHA:AES128-GCM-SHA256:AES128-SHA256:AES128-SHA\&quot;&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


