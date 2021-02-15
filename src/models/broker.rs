/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any combination of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written.|See note 3 Write-Only|Attribute can only be written, not read, unless the attribute is also opaque|See the documentation for the opaque property Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version| Opaque|Attribute can be set or retrieved in opaque form when the `opaquePassword` query parameter is present|See the `opaquePassword` query parameter documentation    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    In the monitoring API, any non-identifying attribute may not be returned in a GET.  ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object (see note 5)|New attribute values|Object attributes and metadata|Set to default, with certain exceptions (see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ### opaquePassword  Attributes with the opaque property are also write-only and so cannot normally be retrieved in a GET. However, when a password is provided in the `opaquePassword` query parameter, attributes with the opaque property are retrieved in a GET in opaque form, encrypted with this password. The query parameter can also be used on a POST, PATCH, or PUT to set opaque attributes using opaque attribute values retrieved in a GET, so long as:  1. the same password that was used to retrieve the opaque attribute values is provided; and  2. the broker to which the request is being sent has the same major and minor SEMP version as the broker that produced the opaque attribute values.  The password provided in the query parameter must be a minimum of 8 characters and a maximum of 128 characters.  The query parameter can only be used in the configuration API, and only over HTTPS.  ## Help  Visit [our website](https://solace.com) to learn more about Solace.  You can also download the SEMP API specifications by clicking [here](https://solace.com/downloads/).  If you need additional support, please contact us at [support@solace.com](mailto:support@solace.com).  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|On a PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT, except in the following two cases: there is a mutual requires relationship with another non-write-only attribute and both attributes are absent from the request; or the attribute is also opaque and the `opaquePassword` query parameter is provided in the request. 5|On a PUT, if the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.19
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Broker {
  /// The client certificate revocation checking mode used when a client authenticates with a client certificate. The default value is `\"none\"`. The allowed values and their meaning are:  <pre> \"none\" - Do not perform any certificate revocation checking. \"ocsp\" - Use the Open Certificate Status Protcol (OCSP) for certificate revocation checking. \"crl\" - Use Certificate Revocation Lists (CRL) for certificate revocation checking. \"ocsp-crl\" - Use OCSP first, but if OCSP fails to return an unambiguous result, then check via CRL. </pre> 
  #[serde(rename = "authClientCertRevocationCheckMode", skip_serializing_if="Option::is_none")]
  auth_client_cert_revocation_check_mode: Option<String>,
  /// The WWN number to use when accessing a LUN on an external disk array. The default value is `\"\"`. Available since 2.18.
  #[serde(rename = "guaranteedMsgingDiskArrayWwn", skip_serializing_if="Option::is_none")]
  guaranteed_msging_disk_array_wwn: Option<String>,
  /// The disk location for the the guaranteed message spool (required for high availability with guaranteed messaging). When external is chosen the guaranteed message spool is stored on an external disk array attached to the router. If internal storage is currently used, changing to external causes message spooling on the router to stop and messages spooled on the internal storage to be deleted. If internal is chosen the guaranteed message spool is stored on an external disk array attached to the router. If internal storage is currently used, changing to external causes message spooling on the router to stop and messages spooled on the internal storage to be deleted. The default value is `\"external\"`. The allowed values and their meaning are:  <pre> \"external\" - The guaranteed message spool is stored on an external disk array attached to the appliance. \"internal\" - The guaranteed message spool is stored internally on the appliance. </pre>  Available since 2.18.
  #[serde(rename = "guaranteedMsgingDiskLocation", skip_serializing_if="Option::is_none")]
  guaranteed_msging_disk_location: Option<String>,
  /// Enable or disable Guaranteed Messaging. The default value is `false`. Available since 2.18.
  #[serde(rename = "guaranteedMsgingEnabled", skip_serializing_if="Option::is_none")]
  guaranteed_msging_enabled: Option<bool>,
  #[serde(rename = "guaranteedMsgingEventCacheUsageThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_cache_usage_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "guaranteedMsgingEventDeliveredUnackedThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_delivered_unacked_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "guaranteedMsgingEventDiskUsageThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_disk_usage_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "guaranteedMsgingEventEgressFlowCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_egress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "guaranteedMsgingEventEndpointCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_endpoint_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "guaranteedMsgingEventIngressFlowCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_ingress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "guaranteedMsgingEventMsgCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_msg_count_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "guaranteedMsgingEventMsgSpoolFileCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_msg_spool_file_count_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "guaranteedMsgingEventMsgSpoolUsageThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_msg_spool_usage_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "guaranteedMsgingEventTransactedSessionCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_transacted_session_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "guaranteedMsgingEventTransactedSessionResourceCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_transacted_session_resource_count_threshold: Option<::models::EventThresholdByPercent>,
  #[serde(rename = "guaranteedMsgingEventTransactionCountThreshold", skip_serializing_if="Option::is_none")]
  guaranteed_msging_event_transaction_count_threshold: Option<::models::EventThreshold>,
  /// Guaranteed messaging cache usage limit. Expressed as a maximum percentage of the NAB's egress queueing. resources that the guaranteed message cache is allowed to use. The default value is `10`. Available since 2.18.
  #[serde(rename = "guaranteedMsgingMaxCacheUsage", skip_serializing_if="Option::is_none")]
  guaranteed_msging_max_cache_usage: Option<i32>,
  /// The maximum total message spool usage allowed across all VPNs on this broker, in megabytes. Recommendation: the maximum value should be less than 90% of the disk space allocated for the guaranteed message spool. The default value is `60000`. Available since 2.18.
  #[serde(rename = "guaranteedMsgingMaxMsgSpoolUsage", skip_serializing_if="Option::is_none")]
  guaranteed_msging_max_msg_spool_usage: Option<i64>,
  /// The replication compatibility mode for the router. The default value is `\"legacy\"`. The allowed values and their meaning are:\"legacy\" - All transactions originated by clients are replicated to the standby site without using transactions.\"transacted\" - All transactions originated by clients are replicated to the standby site using transactions. The default value is `\"legacy\"`. The allowed values and their meaning are:  <pre> \"legacy\" - All transactions originated by clients are replicated to the standby site without using transactions. \"transacted\" - All transactions originated by clients are replicated to the standby site using transactions. </pre>  Available since 2.18.
  #[serde(rename = "guaranteedMsgingTransactionReplicationCompatibilityMode", skip_serializing_if="Option::is_none")]
  guaranteed_msging_transaction_replication_compatibility_mode: Option<String>,
  /// The High Availability role for this broker if using the legacy Active/Active configuration for high availability (not recommended). Note: for Active/Standby high availability configuration, this setting is ignored. The default value is `\"primary\"`. The allowed values and their meaning are:  <pre> \"primary\" - The primary virtual router. \"backup\" - The backup virtual router. </pre>  Available since 2.18.
  #[serde(rename = "guaranteedMsgingVirtualRouterWhenActiveActive", skip_serializing_if="Option::is_none")]
  guaranteed_msging_virtual_router_when_active_active: Option<String>,
  /// Enable or disable the AMQP service. When disabled new AMQP Clients may not connect through the global or per-VPN AMQP listen-ports, and all currently connected AMQP Clients are immediately disconnected. The default value is `false`. Available since 2.17.
  #[serde(rename = "serviceAmqpEnabled", skip_serializing_if="Option::is_none")]
  service_amqp_enabled: Option<bool>,
  /// TCP port number that AMQP clients can use to connect to the broker using raw TCP over TLS. The default value is `0`. Available since 2.17.
  #[serde(rename = "serviceAmqpTlsListenPort", skip_serializing_if="Option::is_none")]
  service_amqp_tls_listen_port: Option<i64>,
  #[serde(rename = "serviceEventConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  service_event_connection_count_threshold: Option<::models::EventThreshold>,
  /// Enable or disable the health-check service. The default value is `false`. Available since 2.17.
  #[serde(rename = "serviceHealthCheckEnabled", skip_serializing_if="Option::is_none")]
  service_health_check_enabled: Option<bool>,
  /// The port number for the health-check service. The port must be unique across the message backbone. The health-check service must be disabled to change the port. The default value is `5550`. Available since 2.17.
  #[serde(rename = "serviceHealthCheckListenPort", skip_serializing_if="Option::is_none")]
  service_health_check_listen_port: Option<i64>,
  /// Enable or disable the MQTT service. When disabled new MQTT Clients may not connect through the per-VPN MQTT listen-ports, and all currently connected MQTT Clients are immediately disconnected. The default value is `false`. Available since 2.17.
  #[serde(rename = "serviceMqttEnabled", skip_serializing_if="Option::is_none")]
  service_mqtt_enabled: Option<bool>,
  /// Enable or disable the msg-backbone service. When disabled new Clients may not connect through global or per-VPN listen-ports, and all currently connected Clients are immediately disconnected. The default value is `true`. Available since 2.17.
  #[serde(rename = "serviceMsgBackboneEnabled", skip_serializing_if="Option::is_none")]
  service_msg_backbone_enabled: Option<bool>,
  #[serde(rename = "serviceRestEventOutgoingConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  service_rest_event_outgoing_connection_count_threshold: Option<::models::EventThreshold>,
  /// Enable or disable the REST service incoming connections on the router. The default value is `false`. Available since 2.17.
  #[serde(rename = "serviceRestIncomingEnabled", skip_serializing_if="Option::is_none")]
  service_rest_incoming_enabled: Option<bool>,
  /// Enable or disable the REST service outgoing connections on the router. The default value is `false`. Available since 2.17.
  #[serde(rename = "serviceRestOutgoingEnabled", skip_serializing_if="Option::is_none")]
  service_rest_outgoing_enabled: Option<bool>,
  /// Enable or disable extended SEMP timeouts for paged GETs. When a request times out, it returns the current page of content, even if the page is not full.  When enabled, the timeout is 60 seconds. When disabled, the timeout is 5 seconds.  The recommended setting is disabled (no legacy-timeout).  This parameter is intended as a temporary workaround to be used until SEMP clients can handle short pages.  This setting will be removed in a future release. The default value is `false`. Available since 2.18.
  #[serde(rename = "serviceSempLegacyTimeoutEnabled", skip_serializing_if="Option::is_none")]
  service_semp_legacy_timeout_enabled: Option<bool>,
  /// Enable or disable plain-text SEMP service. The default value is `true`. Available since 2.17.
  #[serde(rename = "serviceSempPlainTextEnabled", skip_serializing_if="Option::is_none")]
  service_semp_plain_text_enabled: Option<bool>,
  /// The TCP port for plain-text SEMP client connections. The default value is `80`. Available since 2.17.
  #[serde(rename = "serviceSempPlainTextListenPort", skip_serializing_if="Option::is_none")]
  service_semp_plain_text_listen_port: Option<i64>,
  /// Enable or disable TLS SEMP service. The default value is `true`. Available since 2.17.
  #[serde(rename = "serviceSempTlsEnabled", skip_serializing_if="Option::is_none")]
  service_semp_tls_enabled: Option<bool>,
  /// The TCP port for TLS SEMP client connections. The default value is `1943`. Available since 2.17.
  #[serde(rename = "serviceSempTlsListenPort", skip_serializing_if="Option::is_none")]
  service_semp_tls_listen_port: Option<i64>,
  /// TCP port number that SMF clients can use to connect to the broker using raw compression TCP. The default value is `55003`. Available since 2.17.
  #[serde(rename = "serviceSmfCompressionListenPort", skip_serializing_if="Option::is_none")]
  service_smf_compression_listen_port: Option<i64>,
  /// Enable or disable the SMF service. When disabled new SMF Clients may not connect through the global listen-ports, and all currently connected SMF Clients are immediately disconnected. The default value is `true`. Available since 2.17.
  #[serde(rename = "serviceSmfEnabled", skip_serializing_if="Option::is_none")]
  service_smf_enabled: Option<bool>,
  #[serde(rename = "serviceSmfEventConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  service_smf_event_connection_count_threshold: Option<::models::EventThreshold>,
  /// TCP port number that SMF clients can use to connect to the broker using raw TCP. The default value is `55555`. Available since 2.17.
  #[serde(rename = "serviceSmfPlainTextListenPort", skip_serializing_if="Option::is_none")]
  service_smf_plain_text_listen_port: Option<i64>,
  /// TCP port number that SMF clients can use to connect to the broker using raw routing control TCP. The default value is `55556`. Available since 2.17.
  #[serde(rename = "serviceSmfRoutingControlListenPort", skip_serializing_if="Option::is_none")]
  service_smf_routing_control_listen_port: Option<i64>,
  /// TCP port number that SMF clients can use to connect to the broker using raw TCP over TLS. The default value is `55443`. Available since 2.17.
  #[serde(rename = "serviceSmfTlsListenPort", skip_serializing_if="Option::is_none")]
  service_smf_tls_listen_port: Option<i64>,
  #[serde(rename = "serviceTlsEventConnectionCountThreshold", skip_serializing_if="Option::is_none")]
  service_tls_event_connection_count_threshold: Option<::models::EventThreshold>,
  /// Enable or disable the web-transport service. When disabled new web-transport Clients may not connect through the global listen-ports, and all currently connected web-transport Clients are immediately disconnected. The default value is `false`. Available since 2.17.
  #[serde(rename = "serviceWebTransportEnabled", skip_serializing_if="Option::is_none")]
  service_web_transport_enabled: Option<bool>,
  /// The TCP port for plain-text WEB client connections. The default value is `80`. Available since 2.17.
  #[serde(rename = "serviceWebTransportPlainTextListenPort", skip_serializing_if="Option::is_none")]
  service_web_transport_plain_text_listen_port: Option<i64>,
  /// The TCP port for TLS WEB client connections. The default value is `443`. Available since 2.17.
  #[serde(rename = "serviceWebTransportTlsListenPort", skip_serializing_if="Option::is_none")]
  service_web_transport_tls_listen_port: Option<i64>,
  /// Used to specify the Web URL suffix that will be used by Web clients when communicating with the broker. The default value is `\"\"`. Available since 2.17.
  #[serde(rename = "serviceWebTransportWebUrlSuffix", skip_serializing_if="Option::is_none")]
  service_web_transport_web_url_suffix: Option<String>,
  /// Enable or disable the blocking of incoming TLS version 1.0 connections. When blocked, existing TLS 1.0 connections from Clients and SEMP users remain connected while new connections are blocked. Note that support for TLS 1.0 will eventually be discontinued, at which time TLS 1.0 connections will be blocked regardless of this setting. The default value is `true`.
  #[serde(rename = "tlsBlockVersion10Enabled", skip_serializing_if="Option::is_none")]
  tls_block_version10_enabled: Option<bool>,
  /// Enable or disable the blocking of TLS version 1.1 connections. When blocked, all existing incoming and outgoing TLS 1.1 connections with Clients, SEMP users, and LDAP servers remain connected while new connections are blocked. Note that support for TLS 1.1 will eventually be discontinued, at which time TLS 1.1 connections will be blocked regardless of this setting. The default value is `false`.
  #[serde(rename = "tlsBlockVersion11Enabled", skip_serializing_if="Option::is_none")]
  tls_block_version11_enabled: Option<bool>,
  /// The colon-separated list of cipher suites used for TLS management connections (e.g. SEMP, LDAP). The value \"default\" implies all supported suites ordered from most secure to least secure. The default value is `\"default\"`.
  #[serde(rename = "tlsCipherSuiteManagementList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_management_list: Option<String>,
  /// The colon-separated list of cipher suites used for TLS data connections (e.g. client pub/sub). The value \"default\" implies all supported suites ordered from most secure to least secure. The default value is `\"default\"`.
  #[serde(rename = "tlsCipherSuiteMsgBackboneList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_msg_backbone_list: Option<String>,
  /// The colon-separated list of cipher suites used for TLS secure shell connections (e.g. SSH, SFTP, SCP). The value \"default\" implies all supported suites ordered from most secure to least secure. The default value is `\"default\"`.
  #[serde(rename = "tlsCipherSuiteSecureShellList", skip_serializing_if="Option::is_none")]
  tls_cipher_suite_secure_shell_list: Option<String>,
  /// Enable or disable protection against the CRIME exploit. When enabled, TLS+compressed messaging performance is degraded. This protection should only be disabled if sufficient ACL and authentication features are being employed such that a potential attacker does not have sufficient access to trigger the exploit. The default value is `true`.
  #[serde(rename = "tlsCrimeExploitProtectionEnabled", skip_serializing_if="Option::is_none")]
  tls_crime_exploit_protection_enabled: Option<bool>,
  /// The PEM formatted content for the server certificate used for TLS connections. It must consist of a private key and between one and three certificates comprising the certificate trust chain. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. Changing this attribute requires an HTTPS connection. The default value is `\"\"`.
  #[serde(rename = "tlsServerCertContent", skip_serializing_if="Option::is_none")]
  tls_server_cert_content: Option<String>,
  /// The password for the server certificate used for TLS connections. This attribute is absent from a GET and not updated when absent in a PUT, subject to the exceptions in note 4. Changing this attribute requires an HTTPS connection. The default value is `\"\"`.
  #[serde(rename = "tlsServerCertPassword", skip_serializing_if="Option::is_none")]
  tls_server_cert_password: Option<String>,
  /// Enable or disable the standard domain certificate authority list. The default value is `true`. Available since 2.19.
  #[serde(rename = "tlsStandardDomainCertificateAuthoritiesEnabled", skip_serializing_if="Option::is_none")]
  tls_standard_domain_certificate_authorities_enabled: Option<bool>,
  /// The TLS ticket lifetime in seconds. When a client connects with TLS, a session with a session ticket is created using the TLS ticket lifetime which determines how long the client has to resume the session. The default value is `86400`.
  #[serde(rename = "tlsTicketLifetime", skip_serializing_if="Option::is_none")]
  tls_ticket_lifetime: Option<i32>
}

impl Broker {
  pub fn new() -> Broker {
    Broker {
      auth_client_cert_revocation_check_mode: None,
      guaranteed_msging_disk_array_wwn: None,
      guaranteed_msging_disk_location: None,
      guaranteed_msging_enabled: None,
      guaranteed_msging_event_cache_usage_threshold: None,
      guaranteed_msging_event_delivered_unacked_threshold: None,
      guaranteed_msging_event_disk_usage_threshold: None,
      guaranteed_msging_event_egress_flow_count_threshold: None,
      guaranteed_msging_event_endpoint_count_threshold: None,
      guaranteed_msging_event_ingress_flow_count_threshold: None,
      guaranteed_msging_event_msg_count_threshold: None,
      guaranteed_msging_event_msg_spool_file_count_threshold: None,
      guaranteed_msging_event_msg_spool_usage_threshold: None,
      guaranteed_msging_event_transacted_session_count_threshold: None,
      guaranteed_msging_event_transacted_session_resource_count_threshold: None,
      guaranteed_msging_event_transaction_count_threshold: None,
      guaranteed_msging_max_cache_usage: None,
      guaranteed_msging_max_msg_spool_usage: None,
      guaranteed_msging_transaction_replication_compatibility_mode: None,
      guaranteed_msging_virtual_router_when_active_active: None,
      service_amqp_enabled: None,
      service_amqp_tls_listen_port: None,
      service_event_connection_count_threshold: None,
      service_health_check_enabled: None,
      service_health_check_listen_port: None,
      service_mqtt_enabled: None,
      service_msg_backbone_enabled: None,
      service_rest_event_outgoing_connection_count_threshold: None,
      service_rest_incoming_enabled: None,
      service_rest_outgoing_enabled: None,
      service_semp_legacy_timeout_enabled: None,
      service_semp_plain_text_enabled: None,
      service_semp_plain_text_listen_port: None,
      service_semp_tls_enabled: None,
      service_semp_tls_listen_port: None,
      service_smf_compression_listen_port: None,
      service_smf_enabled: None,
      service_smf_event_connection_count_threshold: None,
      service_smf_plain_text_listen_port: None,
      service_smf_routing_control_listen_port: None,
      service_smf_tls_listen_port: None,
      service_tls_event_connection_count_threshold: None,
      service_web_transport_enabled: None,
      service_web_transport_plain_text_listen_port: None,
      service_web_transport_tls_listen_port: None,
      service_web_transport_web_url_suffix: None,
      tls_block_version10_enabled: None,
      tls_block_version11_enabled: None,
      tls_cipher_suite_management_list: None,
      tls_cipher_suite_msg_backbone_list: None,
      tls_cipher_suite_secure_shell_list: None,
      tls_crime_exploit_protection_enabled: None,
      tls_server_cert_content: None,
      tls_server_cert_password: None,
      tls_standard_domain_certificate_authorities_enabled: None,
      tls_ticket_lifetime: None
    }
  }

  pub fn set_auth_client_cert_revocation_check_mode(&mut self, auth_client_cert_revocation_check_mode: String) {
    self.auth_client_cert_revocation_check_mode = Some(auth_client_cert_revocation_check_mode);
  }

  pub fn with_auth_client_cert_revocation_check_mode(mut self, auth_client_cert_revocation_check_mode: String) -> Broker {
    self.auth_client_cert_revocation_check_mode = Some(auth_client_cert_revocation_check_mode);
    self
  }

  pub fn auth_client_cert_revocation_check_mode(&self) -> Option<&String> {
    self.auth_client_cert_revocation_check_mode.as_ref()
  }

  pub fn reset_auth_client_cert_revocation_check_mode(&mut self) {
    self.auth_client_cert_revocation_check_mode = None;
  }

  pub fn set_guaranteed_msging_disk_array_wwn(&mut self, guaranteed_msging_disk_array_wwn: String) {
    self.guaranteed_msging_disk_array_wwn = Some(guaranteed_msging_disk_array_wwn);
  }

  pub fn with_guaranteed_msging_disk_array_wwn(mut self, guaranteed_msging_disk_array_wwn: String) -> Broker {
    self.guaranteed_msging_disk_array_wwn = Some(guaranteed_msging_disk_array_wwn);
    self
  }

  pub fn guaranteed_msging_disk_array_wwn(&self) -> Option<&String> {
    self.guaranteed_msging_disk_array_wwn.as_ref()
  }

  pub fn reset_guaranteed_msging_disk_array_wwn(&mut self) {
    self.guaranteed_msging_disk_array_wwn = None;
  }

  pub fn set_guaranteed_msging_disk_location(&mut self, guaranteed_msging_disk_location: String) {
    self.guaranteed_msging_disk_location = Some(guaranteed_msging_disk_location);
  }

  pub fn with_guaranteed_msging_disk_location(mut self, guaranteed_msging_disk_location: String) -> Broker {
    self.guaranteed_msging_disk_location = Some(guaranteed_msging_disk_location);
    self
  }

  pub fn guaranteed_msging_disk_location(&self) -> Option<&String> {
    self.guaranteed_msging_disk_location.as_ref()
  }

  pub fn reset_guaranteed_msging_disk_location(&mut self) {
    self.guaranteed_msging_disk_location = None;
  }

  pub fn set_guaranteed_msging_enabled(&mut self, guaranteed_msging_enabled: bool) {
    self.guaranteed_msging_enabled = Some(guaranteed_msging_enabled);
  }

  pub fn with_guaranteed_msging_enabled(mut self, guaranteed_msging_enabled: bool) -> Broker {
    self.guaranteed_msging_enabled = Some(guaranteed_msging_enabled);
    self
  }

  pub fn guaranteed_msging_enabled(&self) -> Option<&bool> {
    self.guaranteed_msging_enabled.as_ref()
  }

  pub fn reset_guaranteed_msging_enabled(&mut self) {
    self.guaranteed_msging_enabled = None;
  }

  pub fn set_guaranteed_msging_event_cache_usage_threshold(&mut self, guaranteed_msging_event_cache_usage_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_cache_usage_threshold = Some(guaranteed_msging_event_cache_usage_threshold);
  }

  pub fn with_guaranteed_msging_event_cache_usage_threshold(mut self, guaranteed_msging_event_cache_usage_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_cache_usage_threshold = Some(guaranteed_msging_event_cache_usage_threshold);
    self
  }

  pub fn guaranteed_msging_event_cache_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_cache_usage_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_cache_usage_threshold(&mut self) {
    self.guaranteed_msging_event_cache_usage_threshold = None;
  }

  pub fn set_guaranteed_msging_event_delivered_unacked_threshold(&mut self, guaranteed_msging_event_delivered_unacked_threshold: ::models::EventThresholdByPercent) {
    self.guaranteed_msging_event_delivered_unacked_threshold = Some(guaranteed_msging_event_delivered_unacked_threshold);
  }

  pub fn with_guaranteed_msging_event_delivered_unacked_threshold(mut self, guaranteed_msging_event_delivered_unacked_threshold: ::models::EventThresholdByPercent) -> Broker {
    self.guaranteed_msging_event_delivered_unacked_threshold = Some(guaranteed_msging_event_delivered_unacked_threshold);
    self
  }

  pub fn guaranteed_msging_event_delivered_unacked_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.guaranteed_msging_event_delivered_unacked_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_delivered_unacked_threshold(&mut self) {
    self.guaranteed_msging_event_delivered_unacked_threshold = None;
  }

  pub fn set_guaranteed_msging_event_disk_usage_threshold(&mut self, guaranteed_msging_event_disk_usage_threshold: ::models::EventThresholdByPercent) {
    self.guaranteed_msging_event_disk_usage_threshold = Some(guaranteed_msging_event_disk_usage_threshold);
  }

  pub fn with_guaranteed_msging_event_disk_usage_threshold(mut self, guaranteed_msging_event_disk_usage_threshold: ::models::EventThresholdByPercent) -> Broker {
    self.guaranteed_msging_event_disk_usage_threshold = Some(guaranteed_msging_event_disk_usage_threshold);
    self
  }

  pub fn guaranteed_msging_event_disk_usage_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.guaranteed_msging_event_disk_usage_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_disk_usage_threshold(&mut self) {
    self.guaranteed_msging_event_disk_usage_threshold = None;
  }

  pub fn set_guaranteed_msging_event_egress_flow_count_threshold(&mut self, guaranteed_msging_event_egress_flow_count_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_egress_flow_count_threshold = Some(guaranteed_msging_event_egress_flow_count_threshold);
  }

  pub fn with_guaranteed_msging_event_egress_flow_count_threshold(mut self, guaranteed_msging_event_egress_flow_count_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_egress_flow_count_threshold = Some(guaranteed_msging_event_egress_flow_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_egress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_egress_flow_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_egress_flow_count_threshold(&mut self) {
    self.guaranteed_msging_event_egress_flow_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_endpoint_count_threshold(&mut self, guaranteed_msging_event_endpoint_count_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_endpoint_count_threshold = Some(guaranteed_msging_event_endpoint_count_threshold);
  }

  pub fn with_guaranteed_msging_event_endpoint_count_threshold(mut self, guaranteed_msging_event_endpoint_count_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_endpoint_count_threshold = Some(guaranteed_msging_event_endpoint_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_endpoint_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_endpoint_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_endpoint_count_threshold(&mut self) {
    self.guaranteed_msging_event_endpoint_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_ingress_flow_count_threshold(&mut self, guaranteed_msging_event_ingress_flow_count_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_ingress_flow_count_threshold = Some(guaranteed_msging_event_ingress_flow_count_threshold);
  }

  pub fn with_guaranteed_msging_event_ingress_flow_count_threshold(mut self, guaranteed_msging_event_ingress_flow_count_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_ingress_flow_count_threshold = Some(guaranteed_msging_event_ingress_flow_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_ingress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_ingress_flow_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_ingress_flow_count_threshold(&mut self) {
    self.guaranteed_msging_event_ingress_flow_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_msg_count_threshold(&mut self, guaranteed_msging_event_msg_count_threshold: ::models::EventThresholdByPercent) {
    self.guaranteed_msging_event_msg_count_threshold = Some(guaranteed_msging_event_msg_count_threshold);
  }

  pub fn with_guaranteed_msging_event_msg_count_threshold(mut self, guaranteed_msging_event_msg_count_threshold: ::models::EventThresholdByPercent) -> Broker {
    self.guaranteed_msging_event_msg_count_threshold = Some(guaranteed_msging_event_msg_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_msg_count_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.guaranteed_msging_event_msg_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_msg_count_threshold(&mut self) {
    self.guaranteed_msging_event_msg_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_msg_spool_file_count_threshold(&mut self, guaranteed_msging_event_msg_spool_file_count_threshold: ::models::EventThresholdByPercent) {
    self.guaranteed_msging_event_msg_spool_file_count_threshold = Some(guaranteed_msging_event_msg_spool_file_count_threshold);
  }

  pub fn with_guaranteed_msging_event_msg_spool_file_count_threshold(mut self, guaranteed_msging_event_msg_spool_file_count_threshold: ::models::EventThresholdByPercent) -> Broker {
    self.guaranteed_msging_event_msg_spool_file_count_threshold = Some(guaranteed_msging_event_msg_spool_file_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_msg_spool_file_count_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.guaranteed_msging_event_msg_spool_file_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_msg_spool_file_count_threshold(&mut self) {
    self.guaranteed_msging_event_msg_spool_file_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_msg_spool_usage_threshold(&mut self, guaranteed_msging_event_msg_spool_usage_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_msg_spool_usage_threshold = Some(guaranteed_msging_event_msg_spool_usage_threshold);
  }

  pub fn with_guaranteed_msging_event_msg_spool_usage_threshold(mut self, guaranteed_msging_event_msg_spool_usage_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_msg_spool_usage_threshold = Some(guaranteed_msging_event_msg_spool_usage_threshold);
    self
  }

  pub fn guaranteed_msging_event_msg_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_msg_spool_usage_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_msg_spool_usage_threshold(&mut self) {
    self.guaranteed_msging_event_msg_spool_usage_threshold = None;
  }

  pub fn set_guaranteed_msging_event_transacted_session_count_threshold(&mut self, guaranteed_msging_event_transacted_session_count_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_transacted_session_count_threshold = Some(guaranteed_msging_event_transacted_session_count_threshold);
  }

  pub fn with_guaranteed_msging_event_transacted_session_count_threshold(mut self, guaranteed_msging_event_transacted_session_count_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_transacted_session_count_threshold = Some(guaranteed_msging_event_transacted_session_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_transacted_session_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_transacted_session_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_transacted_session_count_threshold(&mut self) {
    self.guaranteed_msging_event_transacted_session_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_transacted_session_resource_count_threshold(&mut self, guaranteed_msging_event_transacted_session_resource_count_threshold: ::models::EventThresholdByPercent) {
    self.guaranteed_msging_event_transacted_session_resource_count_threshold = Some(guaranteed_msging_event_transacted_session_resource_count_threshold);
  }

  pub fn with_guaranteed_msging_event_transacted_session_resource_count_threshold(mut self, guaranteed_msging_event_transacted_session_resource_count_threshold: ::models::EventThresholdByPercent) -> Broker {
    self.guaranteed_msging_event_transacted_session_resource_count_threshold = Some(guaranteed_msging_event_transacted_session_resource_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_transacted_session_resource_count_threshold(&self) -> Option<&::models::EventThresholdByPercent> {
    self.guaranteed_msging_event_transacted_session_resource_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_transacted_session_resource_count_threshold(&mut self) {
    self.guaranteed_msging_event_transacted_session_resource_count_threshold = None;
  }

  pub fn set_guaranteed_msging_event_transaction_count_threshold(&mut self, guaranteed_msging_event_transaction_count_threshold: ::models::EventThreshold) {
    self.guaranteed_msging_event_transaction_count_threshold = Some(guaranteed_msging_event_transaction_count_threshold);
  }

  pub fn with_guaranteed_msging_event_transaction_count_threshold(mut self, guaranteed_msging_event_transaction_count_threshold: ::models::EventThreshold) -> Broker {
    self.guaranteed_msging_event_transaction_count_threshold = Some(guaranteed_msging_event_transaction_count_threshold);
    self
  }

  pub fn guaranteed_msging_event_transaction_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.guaranteed_msging_event_transaction_count_threshold.as_ref()
  }

  pub fn reset_guaranteed_msging_event_transaction_count_threshold(&mut self) {
    self.guaranteed_msging_event_transaction_count_threshold = None;
  }

  pub fn set_guaranteed_msging_max_cache_usage(&mut self, guaranteed_msging_max_cache_usage: i32) {
    self.guaranteed_msging_max_cache_usage = Some(guaranteed_msging_max_cache_usage);
  }

  pub fn with_guaranteed_msging_max_cache_usage(mut self, guaranteed_msging_max_cache_usage: i32) -> Broker {
    self.guaranteed_msging_max_cache_usage = Some(guaranteed_msging_max_cache_usage);
    self
  }

  pub fn guaranteed_msging_max_cache_usage(&self) -> Option<&i32> {
    self.guaranteed_msging_max_cache_usage.as_ref()
  }

  pub fn reset_guaranteed_msging_max_cache_usage(&mut self) {
    self.guaranteed_msging_max_cache_usage = None;
  }

  pub fn set_guaranteed_msging_max_msg_spool_usage(&mut self, guaranteed_msging_max_msg_spool_usage: i64) {
    self.guaranteed_msging_max_msg_spool_usage = Some(guaranteed_msging_max_msg_spool_usage);
  }

  pub fn with_guaranteed_msging_max_msg_spool_usage(mut self, guaranteed_msging_max_msg_spool_usage: i64) -> Broker {
    self.guaranteed_msging_max_msg_spool_usage = Some(guaranteed_msging_max_msg_spool_usage);
    self
  }

  pub fn guaranteed_msging_max_msg_spool_usage(&self) -> Option<&i64> {
    self.guaranteed_msging_max_msg_spool_usage.as_ref()
  }

  pub fn reset_guaranteed_msging_max_msg_spool_usage(&mut self) {
    self.guaranteed_msging_max_msg_spool_usage = None;
  }

  pub fn set_guaranteed_msging_transaction_replication_compatibility_mode(&mut self, guaranteed_msging_transaction_replication_compatibility_mode: String) {
    self.guaranteed_msging_transaction_replication_compatibility_mode = Some(guaranteed_msging_transaction_replication_compatibility_mode);
  }

  pub fn with_guaranteed_msging_transaction_replication_compatibility_mode(mut self, guaranteed_msging_transaction_replication_compatibility_mode: String) -> Broker {
    self.guaranteed_msging_transaction_replication_compatibility_mode = Some(guaranteed_msging_transaction_replication_compatibility_mode);
    self
  }

  pub fn guaranteed_msging_transaction_replication_compatibility_mode(&self) -> Option<&String> {
    self.guaranteed_msging_transaction_replication_compatibility_mode.as_ref()
  }

  pub fn reset_guaranteed_msging_transaction_replication_compatibility_mode(&mut self) {
    self.guaranteed_msging_transaction_replication_compatibility_mode = None;
  }

  pub fn set_guaranteed_msging_virtual_router_when_active_active(&mut self, guaranteed_msging_virtual_router_when_active_active: String) {
    self.guaranteed_msging_virtual_router_when_active_active = Some(guaranteed_msging_virtual_router_when_active_active);
  }

  pub fn with_guaranteed_msging_virtual_router_when_active_active(mut self, guaranteed_msging_virtual_router_when_active_active: String) -> Broker {
    self.guaranteed_msging_virtual_router_when_active_active = Some(guaranteed_msging_virtual_router_when_active_active);
    self
  }

  pub fn guaranteed_msging_virtual_router_when_active_active(&self) -> Option<&String> {
    self.guaranteed_msging_virtual_router_when_active_active.as_ref()
  }

  pub fn reset_guaranteed_msging_virtual_router_when_active_active(&mut self) {
    self.guaranteed_msging_virtual_router_when_active_active = None;
  }

  pub fn set_service_amqp_enabled(&mut self, service_amqp_enabled: bool) {
    self.service_amqp_enabled = Some(service_amqp_enabled);
  }

  pub fn with_service_amqp_enabled(mut self, service_amqp_enabled: bool) -> Broker {
    self.service_amqp_enabled = Some(service_amqp_enabled);
    self
  }

  pub fn service_amqp_enabled(&self) -> Option<&bool> {
    self.service_amqp_enabled.as_ref()
  }

  pub fn reset_service_amqp_enabled(&mut self) {
    self.service_amqp_enabled = None;
  }

  pub fn set_service_amqp_tls_listen_port(&mut self, service_amqp_tls_listen_port: i64) {
    self.service_amqp_tls_listen_port = Some(service_amqp_tls_listen_port);
  }

  pub fn with_service_amqp_tls_listen_port(mut self, service_amqp_tls_listen_port: i64) -> Broker {
    self.service_amqp_tls_listen_port = Some(service_amqp_tls_listen_port);
    self
  }

  pub fn service_amqp_tls_listen_port(&self) -> Option<&i64> {
    self.service_amqp_tls_listen_port.as_ref()
  }

  pub fn reset_service_amqp_tls_listen_port(&mut self) {
    self.service_amqp_tls_listen_port = None;
  }

  pub fn set_service_event_connection_count_threshold(&mut self, service_event_connection_count_threshold: ::models::EventThreshold) {
    self.service_event_connection_count_threshold = Some(service_event_connection_count_threshold);
  }

  pub fn with_service_event_connection_count_threshold(mut self, service_event_connection_count_threshold: ::models::EventThreshold) -> Broker {
    self.service_event_connection_count_threshold = Some(service_event_connection_count_threshold);
    self
  }

  pub fn service_event_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.service_event_connection_count_threshold.as_ref()
  }

  pub fn reset_service_event_connection_count_threshold(&mut self) {
    self.service_event_connection_count_threshold = None;
  }

  pub fn set_service_health_check_enabled(&mut self, service_health_check_enabled: bool) {
    self.service_health_check_enabled = Some(service_health_check_enabled);
  }

  pub fn with_service_health_check_enabled(mut self, service_health_check_enabled: bool) -> Broker {
    self.service_health_check_enabled = Some(service_health_check_enabled);
    self
  }

  pub fn service_health_check_enabled(&self) -> Option<&bool> {
    self.service_health_check_enabled.as_ref()
  }

  pub fn reset_service_health_check_enabled(&mut self) {
    self.service_health_check_enabled = None;
  }

  pub fn set_service_health_check_listen_port(&mut self, service_health_check_listen_port: i64) {
    self.service_health_check_listen_port = Some(service_health_check_listen_port);
  }

  pub fn with_service_health_check_listen_port(mut self, service_health_check_listen_port: i64) -> Broker {
    self.service_health_check_listen_port = Some(service_health_check_listen_port);
    self
  }

  pub fn service_health_check_listen_port(&self) -> Option<&i64> {
    self.service_health_check_listen_port.as_ref()
  }

  pub fn reset_service_health_check_listen_port(&mut self) {
    self.service_health_check_listen_port = None;
  }

  pub fn set_service_mqtt_enabled(&mut self, service_mqtt_enabled: bool) {
    self.service_mqtt_enabled = Some(service_mqtt_enabled);
  }

  pub fn with_service_mqtt_enabled(mut self, service_mqtt_enabled: bool) -> Broker {
    self.service_mqtt_enabled = Some(service_mqtt_enabled);
    self
  }

  pub fn service_mqtt_enabled(&self) -> Option<&bool> {
    self.service_mqtt_enabled.as_ref()
  }

  pub fn reset_service_mqtt_enabled(&mut self) {
    self.service_mqtt_enabled = None;
  }

  pub fn set_service_msg_backbone_enabled(&mut self, service_msg_backbone_enabled: bool) {
    self.service_msg_backbone_enabled = Some(service_msg_backbone_enabled);
  }

  pub fn with_service_msg_backbone_enabled(mut self, service_msg_backbone_enabled: bool) -> Broker {
    self.service_msg_backbone_enabled = Some(service_msg_backbone_enabled);
    self
  }

  pub fn service_msg_backbone_enabled(&self) -> Option<&bool> {
    self.service_msg_backbone_enabled.as_ref()
  }

  pub fn reset_service_msg_backbone_enabled(&mut self) {
    self.service_msg_backbone_enabled = None;
  }

  pub fn set_service_rest_event_outgoing_connection_count_threshold(&mut self, service_rest_event_outgoing_connection_count_threshold: ::models::EventThreshold) {
    self.service_rest_event_outgoing_connection_count_threshold = Some(service_rest_event_outgoing_connection_count_threshold);
  }

  pub fn with_service_rest_event_outgoing_connection_count_threshold(mut self, service_rest_event_outgoing_connection_count_threshold: ::models::EventThreshold) -> Broker {
    self.service_rest_event_outgoing_connection_count_threshold = Some(service_rest_event_outgoing_connection_count_threshold);
    self
  }

  pub fn service_rest_event_outgoing_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.service_rest_event_outgoing_connection_count_threshold.as_ref()
  }

  pub fn reset_service_rest_event_outgoing_connection_count_threshold(&mut self) {
    self.service_rest_event_outgoing_connection_count_threshold = None;
  }

  pub fn set_service_rest_incoming_enabled(&mut self, service_rest_incoming_enabled: bool) {
    self.service_rest_incoming_enabled = Some(service_rest_incoming_enabled);
  }

  pub fn with_service_rest_incoming_enabled(mut self, service_rest_incoming_enabled: bool) -> Broker {
    self.service_rest_incoming_enabled = Some(service_rest_incoming_enabled);
    self
  }

  pub fn service_rest_incoming_enabled(&self) -> Option<&bool> {
    self.service_rest_incoming_enabled.as_ref()
  }

  pub fn reset_service_rest_incoming_enabled(&mut self) {
    self.service_rest_incoming_enabled = None;
  }

  pub fn set_service_rest_outgoing_enabled(&mut self, service_rest_outgoing_enabled: bool) {
    self.service_rest_outgoing_enabled = Some(service_rest_outgoing_enabled);
  }

  pub fn with_service_rest_outgoing_enabled(mut self, service_rest_outgoing_enabled: bool) -> Broker {
    self.service_rest_outgoing_enabled = Some(service_rest_outgoing_enabled);
    self
  }

  pub fn service_rest_outgoing_enabled(&self) -> Option<&bool> {
    self.service_rest_outgoing_enabled.as_ref()
  }

  pub fn reset_service_rest_outgoing_enabled(&mut self) {
    self.service_rest_outgoing_enabled = None;
  }

  pub fn set_service_semp_legacy_timeout_enabled(&mut self, service_semp_legacy_timeout_enabled: bool) {
    self.service_semp_legacy_timeout_enabled = Some(service_semp_legacy_timeout_enabled);
  }

  pub fn with_service_semp_legacy_timeout_enabled(mut self, service_semp_legacy_timeout_enabled: bool) -> Broker {
    self.service_semp_legacy_timeout_enabled = Some(service_semp_legacy_timeout_enabled);
    self
  }

  pub fn service_semp_legacy_timeout_enabled(&self) -> Option<&bool> {
    self.service_semp_legacy_timeout_enabled.as_ref()
  }

  pub fn reset_service_semp_legacy_timeout_enabled(&mut self) {
    self.service_semp_legacy_timeout_enabled = None;
  }

  pub fn set_service_semp_plain_text_enabled(&mut self, service_semp_plain_text_enabled: bool) {
    self.service_semp_plain_text_enabled = Some(service_semp_plain_text_enabled);
  }

  pub fn with_service_semp_plain_text_enabled(mut self, service_semp_plain_text_enabled: bool) -> Broker {
    self.service_semp_plain_text_enabled = Some(service_semp_plain_text_enabled);
    self
  }

  pub fn service_semp_plain_text_enabled(&self) -> Option<&bool> {
    self.service_semp_plain_text_enabled.as_ref()
  }

  pub fn reset_service_semp_plain_text_enabled(&mut self) {
    self.service_semp_plain_text_enabled = None;
  }

  pub fn set_service_semp_plain_text_listen_port(&mut self, service_semp_plain_text_listen_port: i64) {
    self.service_semp_plain_text_listen_port = Some(service_semp_plain_text_listen_port);
  }

  pub fn with_service_semp_plain_text_listen_port(mut self, service_semp_plain_text_listen_port: i64) -> Broker {
    self.service_semp_plain_text_listen_port = Some(service_semp_plain_text_listen_port);
    self
  }

  pub fn service_semp_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_semp_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_semp_plain_text_listen_port(&mut self) {
    self.service_semp_plain_text_listen_port = None;
  }

  pub fn set_service_semp_tls_enabled(&mut self, service_semp_tls_enabled: bool) {
    self.service_semp_tls_enabled = Some(service_semp_tls_enabled);
  }

  pub fn with_service_semp_tls_enabled(mut self, service_semp_tls_enabled: bool) -> Broker {
    self.service_semp_tls_enabled = Some(service_semp_tls_enabled);
    self
  }

  pub fn service_semp_tls_enabled(&self) -> Option<&bool> {
    self.service_semp_tls_enabled.as_ref()
  }

  pub fn reset_service_semp_tls_enabled(&mut self) {
    self.service_semp_tls_enabled = None;
  }

  pub fn set_service_semp_tls_listen_port(&mut self, service_semp_tls_listen_port: i64) {
    self.service_semp_tls_listen_port = Some(service_semp_tls_listen_port);
  }

  pub fn with_service_semp_tls_listen_port(mut self, service_semp_tls_listen_port: i64) -> Broker {
    self.service_semp_tls_listen_port = Some(service_semp_tls_listen_port);
    self
  }

  pub fn service_semp_tls_listen_port(&self) -> Option<&i64> {
    self.service_semp_tls_listen_port.as_ref()
  }

  pub fn reset_service_semp_tls_listen_port(&mut self) {
    self.service_semp_tls_listen_port = None;
  }

  pub fn set_service_smf_compression_listen_port(&mut self, service_smf_compression_listen_port: i64) {
    self.service_smf_compression_listen_port = Some(service_smf_compression_listen_port);
  }

  pub fn with_service_smf_compression_listen_port(mut self, service_smf_compression_listen_port: i64) -> Broker {
    self.service_smf_compression_listen_port = Some(service_smf_compression_listen_port);
    self
  }

  pub fn service_smf_compression_listen_port(&self) -> Option<&i64> {
    self.service_smf_compression_listen_port.as_ref()
  }

  pub fn reset_service_smf_compression_listen_port(&mut self) {
    self.service_smf_compression_listen_port = None;
  }

  pub fn set_service_smf_enabled(&mut self, service_smf_enabled: bool) {
    self.service_smf_enabled = Some(service_smf_enabled);
  }

  pub fn with_service_smf_enabled(mut self, service_smf_enabled: bool) -> Broker {
    self.service_smf_enabled = Some(service_smf_enabled);
    self
  }

  pub fn service_smf_enabled(&self) -> Option<&bool> {
    self.service_smf_enabled.as_ref()
  }

  pub fn reset_service_smf_enabled(&mut self) {
    self.service_smf_enabled = None;
  }

  pub fn set_service_smf_event_connection_count_threshold(&mut self, service_smf_event_connection_count_threshold: ::models::EventThreshold) {
    self.service_smf_event_connection_count_threshold = Some(service_smf_event_connection_count_threshold);
  }

  pub fn with_service_smf_event_connection_count_threshold(mut self, service_smf_event_connection_count_threshold: ::models::EventThreshold) -> Broker {
    self.service_smf_event_connection_count_threshold = Some(service_smf_event_connection_count_threshold);
    self
  }

  pub fn service_smf_event_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.service_smf_event_connection_count_threshold.as_ref()
  }

  pub fn reset_service_smf_event_connection_count_threshold(&mut self) {
    self.service_smf_event_connection_count_threshold = None;
  }

  pub fn set_service_smf_plain_text_listen_port(&mut self, service_smf_plain_text_listen_port: i64) {
    self.service_smf_plain_text_listen_port = Some(service_smf_plain_text_listen_port);
  }

  pub fn with_service_smf_plain_text_listen_port(mut self, service_smf_plain_text_listen_port: i64) -> Broker {
    self.service_smf_plain_text_listen_port = Some(service_smf_plain_text_listen_port);
    self
  }

  pub fn service_smf_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_smf_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_smf_plain_text_listen_port(&mut self) {
    self.service_smf_plain_text_listen_port = None;
  }

  pub fn set_service_smf_routing_control_listen_port(&mut self, service_smf_routing_control_listen_port: i64) {
    self.service_smf_routing_control_listen_port = Some(service_smf_routing_control_listen_port);
  }

  pub fn with_service_smf_routing_control_listen_port(mut self, service_smf_routing_control_listen_port: i64) -> Broker {
    self.service_smf_routing_control_listen_port = Some(service_smf_routing_control_listen_port);
    self
  }

  pub fn service_smf_routing_control_listen_port(&self) -> Option<&i64> {
    self.service_smf_routing_control_listen_port.as_ref()
  }

  pub fn reset_service_smf_routing_control_listen_port(&mut self) {
    self.service_smf_routing_control_listen_port = None;
  }

  pub fn set_service_smf_tls_listen_port(&mut self, service_smf_tls_listen_port: i64) {
    self.service_smf_tls_listen_port = Some(service_smf_tls_listen_port);
  }

  pub fn with_service_smf_tls_listen_port(mut self, service_smf_tls_listen_port: i64) -> Broker {
    self.service_smf_tls_listen_port = Some(service_smf_tls_listen_port);
    self
  }

  pub fn service_smf_tls_listen_port(&self) -> Option<&i64> {
    self.service_smf_tls_listen_port.as_ref()
  }

  pub fn reset_service_smf_tls_listen_port(&mut self) {
    self.service_smf_tls_listen_port = None;
  }

  pub fn set_service_tls_event_connection_count_threshold(&mut self, service_tls_event_connection_count_threshold: ::models::EventThreshold) {
    self.service_tls_event_connection_count_threshold = Some(service_tls_event_connection_count_threshold);
  }

  pub fn with_service_tls_event_connection_count_threshold(mut self, service_tls_event_connection_count_threshold: ::models::EventThreshold) -> Broker {
    self.service_tls_event_connection_count_threshold = Some(service_tls_event_connection_count_threshold);
    self
  }

  pub fn service_tls_event_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.service_tls_event_connection_count_threshold.as_ref()
  }

  pub fn reset_service_tls_event_connection_count_threshold(&mut self) {
    self.service_tls_event_connection_count_threshold = None;
  }

  pub fn set_service_web_transport_enabled(&mut self, service_web_transport_enabled: bool) {
    self.service_web_transport_enabled = Some(service_web_transport_enabled);
  }

  pub fn with_service_web_transport_enabled(mut self, service_web_transport_enabled: bool) -> Broker {
    self.service_web_transport_enabled = Some(service_web_transport_enabled);
    self
  }

  pub fn service_web_transport_enabled(&self) -> Option<&bool> {
    self.service_web_transport_enabled.as_ref()
  }

  pub fn reset_service_web_transport_enabled(&mut self) {
    self.service_web_transport_enabled = None;
  }

  pub fn set_service_web_transport_plain_text_listen_port(&mut self, service_web_transport_plain_text_listen_port: i64) {
    self.service_web_transport_plain_text_listen_port = Some(service_web_transport_plain_text_listen_port);
  }

  pub fn with_service_web_transport_plain_text_listen_port(mut self, service_web_transport_plain_text_listen_port: i64) -> Broker {
    self.service_web_transport_plain_text_listen_port = Some(service_web_transport_plain_text_listen_port);
    self
  }

  pub fn service_web_transport_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_web_transport_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_web_transport_plain_text_listen_port(&mut self) {
    self.service_web_transport_plain_text_listen_port = None;
  }

  pub fn set_service_web_transport_tls_listen_port(&mut self, service_web_transport_tls_listen_port: i64) {
    self.service_web_transport_tls_listen_port = Some(service_web_transport_tls_listen_port);
  }

  pub fn with_service_web_transport_tls_listen_port(mut self, service_web_transport_tls_listen_port: i64) -> Broker {
    self.service_web_transport_tls_listen_port = Some(service_web_transport_tls_listen_port);
    self
  }

  pub fn service_web_transport_tls_listen_port(&self) -> Option<&i64> {
    self.service_web_transport_tls_listen_port.as_ref()
  }

  pub fn reset_service_web_transport_tls_listen_port(&mut self) {
    self.service_web_transport_tls_listen_port = None;
  }

  pub fn set_service_web_transport_web_url_suffix(&mut self, service_web_transport_web_url_suffix: String) {
    self.service_web_transport_web_url_suffix = Some(service_web_transport_web_url_suffix);
  }

  pub fn with_service_web_transport_web_url_suffix(mut self, service_web_transport_web_url_suffix: String) -> Broker {
    self.service_web_transport_web_url_suffix = Some(service_web_transport_web_url_suffix);
    self
  }

  pub fn service_web_transport_web_url_suffix(&self) -> Option<&String> {
    self.service_web_transport_web_url_suffix.as_ref()
  }

  pub fn reset_service_web_transport_web_url_suffix(&mut self) {
    self.service_web_transport_web_url_suffix = None;
  }

  pub fn set_tls_block_version10_enabled(&mut self, tls_block_version10_enabled: bool) {
    self.tls_block_version10_enabled = Some(tls_block_version10_enabled);
  }

  pub fn with_tls_block_version10_enabled(mut self, tls_block_version10_enabled: bool) -> Broker {
    self.tls_block_version10_enabled = Some(tls_block_version10_enabled);
    self
  }

  pub fn tls_block_version10_enabled(&self) -> Option<&bool> {
    self.tls_block_version10_enabled.as_ref()
  }

  pub fn reset_tls_block_version10_enabled(&mut self) {
    self.tls_block_version10_enabled = None;
  }

  pub fn set_tls_block_version11_enabled(&mut self, tls_block_version11_enabled: bool) {
    self.tls_block_version11_enabled = Some(tls_block_version11_enabled);
  }

  pub fn with_tls_block_version11_enabled(mut self, tls_block_version11_enabled: bool) -> Broker {
    self.tls_block_version11_enabled = Some(tls_block_version11_enabled);
    self
  }

  pub fn tls_block_version11_enabled(&self) -> Option<&bool> {
    self.tls_block_version11_enabled.as_ref()
  }

  pub fn reset_tls_block_version11_enabled(&mut self) {
    self.tls_block_version11_enabled = None;
  }

  pub fn set_tls_cipher_suite_management_list(&mut self, tls_cipher_suite_management_list: String) {
    self.tls_cipher_suite_management_list = Some(tls_cipher_suite_management_list);
  }

  pub fn with_tls_cipher_suite_management_list(mut self, tls_cipher_suite_management_list: String) -> Broker {
    self.tls_cipher_suite_management_list = Some(tls_cipher_suite_management_list);
    self
  }

  pub fn tls_cipher_suite_management_list(&self) -> Option<&String> {
    self.tls_cipher_suite_management_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_management_list(&mut self) {
    self.tls_cipher_suite_management_list = None;
  }

  pub fn set_tls_cipher_suite_msg_backbone_list(&mut self, tls_cipher_suite_msg_backbone_list: String) {
    self.tls_cipher_suite_msg_backbone_list = Some(tls_cipher_suite_msg_backbone_list);
  }

  pub fn with_tls_cipher_suite_msg_backbone_list(mut self, tls_cipher_suite_msg_backbone_list: String) -> Broker {
    self.tls_cipher_suite_msg_backbone_list = Some(tls_cipher_suite_msg_backbone_list);
    self
  }

  pub fn tls_cipher_suite_msg_backbone_list(&self) -> Option<&String> {
    self.tls_cipher_suite_msg_backbone_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_msg_backbone_list(&mut self) {
    self.tls_cipher_suite_msg_backbone_list = None;
  }

  pub fn set_tls_cipher_suite_secure_shell_list(&mut self, tls_cipher_suite_secure_shell_list: String) {
    self.tls_cipher_suite_secure_shell_list = Some(tls_cipher_suite_secure_shell_list);
  }

  pub fn with_tls_cipher_suite_secure_shell_list(mut self, tls_cipher_suite_secure_shell_list: String) -> Broker {
    self.tls_cipher_suite_secure_shell_list = Some(tls_cipher_suite_secure_shell_list);
    self
  }

  pub fn tls_cipher_suite_secure_shell_list(&self) -> Option<&String> {
    self.tls_cipher_suite_secure_shell_list.as_ref()
  }

  pub fn reset_tls_cipher_suite_secure_shell_list(&mut self) {
    self.tls_cipher_suite_secure_shell_list = None;
  }

  pub fn set_tls_crime_exploit_protection_enabled(&mut self, tls_crime_exploit_protection_enabled: bool) {
    self.tls_crime_exploit_protection_enabled = Some(tls_crime_exploit_protection_enabled);
  }

  pub fn with_tls_crime_exploit_protection_enabled(mut self, tls_crime_exploit_protection_enabled: bool) -> Broker {
    self.tls_crime_exploit_protection_enabled = Some(tls_crime_exploit_protection_enabled);
    self
  }

  pub fn tls_crime_exploit_protection_enabled(&self) -> Option<&bool> {
    self.tls_crime_exploit_protection_enabled.as_ref()
  }

  pub fn reset_tls_crime_exploit_protection_enabled(&mut self) {
    self.tls_crime_exploit_protection_enabled = None;
  }

  pub fn set_tls_server_cert_content(&mut self, tls_server_cert_content: String) {
    self.tls_server_cert_content = Some(tls_server_cert_content);
  }

  pub fn with_tls_server_cert_content(mut self, tls_server_cert_content: String) -> Broker {
    self.tls_server_cert_content = Some(tls_server_cert_content);
    self
  }

  pub fn tls_server_cert_content(&self) -> Option<&String> {
    self.tls_server_cert_content.as_ref()
  }

  pub fn reset_tls_server_cert_content(&mut self) {
    self.tls_server_cert_content = None;
  }

  pub fn set_tls_server_cert_password(&mut self, tls_server_cert_password: String) {
    self.tls_server_cert_password = Some(tls_server_cert_password);
  }

  pub fn with_tls_server_cert_password(mut self, tls_server_cert_password: String) -> Broker {
    self.tls_server_cert_password = Some(tls_server_cert_password);
    self
  }

  pub fn tls_server_cert_password(&self) -> Option<&String> {
    self.tls_server_cert_password.as_ref()
  }

  pub fn reset_tls_server_cert_password(&mut self) {
    self.tls_server_cert_password = None;
  }

  pub fn set_tls_standard_domain_certificate_authorities_enabled(&mut self, tls_standard_domain_certificate_authorities_enabled: bool) {
    self.tls_standard_domain_certificate_authorities_enabled = Some(tls_standard_domain_certificate_authorities_enabled);
  }

  pub fn with_tls_standard_domain_certificate_authorities_enabled(mut self, tls_standard_domain_certificate_authorities_enabled: bool) -> Broker {
    self.tls_standard_domain_certificate_authorities_enabled = Some(tls_standard_domain_certificate_authorities_enabled);
    self
  }

  pub fn tls_standard_domain_certificate_authorities_enabled(&self) -> Option<&bool> {
    self.tls_standard_domain_certificate_authorities_enabled.as_ref()
  }

  pub fn reset_tls_standard_domain_certificate_authorities_enabled(&mut self) {
    self.tls_standard_domain_certificate_authorities_enabled = None;
  }

  pub fn set_tls_ticket_lifetime(&mut self, tls_ticket_lifetime: i32) {
    self.tls_ticket_lifetime = Some(tls_ticket_lifetime);
  }

  pub fn with_tls_ticket_lifetime(mut self, tls_ticket_lifetime: i32) -> Broker {
    self.tls_ticket_lifetime = Some(tls_ticket_lifetime);
    self
  }

  pub fn tls_ticket_lifetime(&self) -> Option<&i32> {
    self.tls_ticket_lifetime.as_ref()
  }

  pub fn reset_tls_ticket_lifetime(&mut self) {
    self.tls_ticket_lifetime = None;
  }

}



