# DmrCluster

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_basic_enabled** | **bool** | Enable or disable basic authentication for Cluster Links. The default value is &#x60;true&#x60;. | [optional] [default to null]
**authentication_basic_password** | **String** | The password used to authenticate incoming Cluster Links when using basic internal authentication. The same password is also used by outgoing Cluster Links if a per-Link password is not configured. The default is to have no &#x60;authenticationBasicPassword&#x60;. | [optional] [default to null]
**authentication_basic_type** | **String** | The type of basic authentication to use for Cluster Links. The default value is &#x60;\&quot;internal\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;internal\&quot; - Use locally configured password. \&quot;none\&quot; - No authentication. &lt;/pre&gt;  | [optional] [default to null]
**authentication_client_cert_content** | **String** | The PEM formatted content for the client certificate used to login to the remote node. It must consist of a private key and between one and three certificates comprising the certificate trust chain. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**authentication_client_cert_enabled** | **bool** | Enable or disable client certificate authentication for Cluster Links. The default value is &#x60;true&#x60;. | [optional] [default to null]
**authentication_client_cert_password** | **String** | The password for the client certificate. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**direct_only_enabled** | **bool** | Enable or disable direct messaging only. Guaranteed messages will not be transmitted through the cluster. | [optional] [default to null]
**dmr_cluster_name** | **String** | The name of the Cluster. | [optional] [default to null]
**enabled** | **bool** | Enable or disable the Cluster. The default value is &#x60;false&#x60;. | [optional] [default to null]
**node_name** | **String** | The name of this node in the Cluster. This is the name that this broker (or redundant group of brokers) is know by to other nodes in the Cluster. The name is chosen automatically to be either this broker&#39;s Router Name or Mate Router Name, depending on which Active Standby Role (primary or backup) this broker plays in its redundancy group. | [optional] [default to null]
**tls_server_cert_enforce_trusted_common_name_enabled** | **bool** | Enable or disable the enforcing of the common-name provided by the remote broker against the list of trusted common-names configured for the Link. If enabled, the certificate&#39;s common-name must match one of the trusted common-names for the Link to be accepted. The default value is &#x60;true&#x60;. | [optional] [default to null]
**tls_server_cert_max_chain_depth** | **i64** | The maximum allowed depth of a certificate chain. The depth of a chain is defined as the number of signing CA certificates that are present in the chain back to a trusted self-signed root CA certificate. The default value is &#x60;3&#x60;. | [optional] [default to null]
**tls_server_cert_validate_date_enabled** | **bool** | Enable or disable the validation of the \&quot;Not Before\&quot; and \&quot;Not After\&quot; validity dates in the certificate. When disabled, the certificate is accepted even if the certificate is not valid based on these dates. The default value is &#x60;true&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


