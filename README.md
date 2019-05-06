# Rust API client for solace_semp_client

SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the  action API.  Attributes in an object or action may have any (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See note 3 Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters \"q1\" and \"q2\" ; with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is a hidden maximum as to prevent overloading the system. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first.    

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 2.11.00901000077
- Package version: 9.1.0-77
- Build package: io.swagger.codegen.languages.RustClientCodegen
For more information, please visit [http://www.solace.com](http://www.solace.com)

## Installation
Put the package under your project folder and add the following in import:
```
    "./solace_semp_client"
```

## Documentation for API Endpoints

All URIs are relative to *http://www.solace.com/SEMP/v2/config*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AboutApi* | [**get_about_api**](docs/AboutApi.md#get_about_api) | **Get** /about/api | Get an API Description object.
*AboutApi* | [**get_about_user**](docs/AboutApi.md#get_about_user) | **Get** /about/user | Get a User object.
*AclProfileApi* | [**get_msg_vpn_acl_profiles**](docs/AclProfileApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Get a list of ACL Profile objects.
*AuthorizationGroupApi* | [**get_msg_vpn_authorization_groups**](docs/AuthorizationGroupApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Get a list of LDAP Authorization Group objects.
*BridgeApi* | [**get_msg_vpn_bridges**](docs/BridgeApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*ClientProfileApi* | [**get_msg_vpn_client_profiles**](docs/ClientProfileApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Get a list of Client Profile objects.
*ClientUsernameApi* | [**get_msg_vpn_client_usernames**](docs/ClientUsernameApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Get a list of Client Username objects.
*DefaultApi* | [**create_dmr_cluster**](docs/DefaultApi.md#create_dmr_cluster) | **Post** /dmrClusters | Create a Cluster object.
*DefaultApi* | [**create_dmr_cluster_link**](docs/DefaultApi.md#create_dmr_cluster_link) | **Post** /dmrClusters/{dmrClusterName}/links | Create a Link object.
*DefaultApi* | [**create_dmr_cluster_link_remote_address**](docs/DefaultApi.md#create_dmr_cluster_link_remote_address) | **Post** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Create a Remote Address object.
*DefaultApi* | [**create_dmr_cluster_link_tls_trusted_common_name**](docs/DefaultApi.md#create_dmr_cluster_link_tls_trusted_common_name) | **Post** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*DefaultApi* | [**create_msg_vpn**](docs/DefaultApi.md#create_msg_vpn) | **Post** /msgVpns | Create a Message VPN object.
*DefaultApi* | [**create_msg_vpn_acl_profile**](docs/DefaultApi.md#create_msg_vpn_acl_profile) | **Post** /msgVpns/{msgVpnName}/aclProfiles | Create an ACL Profile object.
*DefaultApi* | [**create_msg_vpn_acl_profile_client_connect_exception**](docs/DefaultApi.md#create_msg_vpn_acl_profile_client_connect_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Create a Client Connect Exception object.
*DefaultApi* | [**create_msg_vpn_acl_profile_publish_exception**](docs/DefaultApi.md#create_msg_vpn_acl_profile_publish_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Create a Publish Topic Exception object.
*DefaultApi* | [**create_msg_vpn_acl_profile_subscribe_exception**](docs/DefaultApi.md#create_msg_vpn_acl_profile_subscribe_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Create a Subscribe Topic Exception object.
*DefaultApi* | [**create_msg_vpn_authorization_group**](docs/DefaultApi.md#create_msg_vpn_authorization_group) | **Post** /msgVpns/{msgVpnName}/authorizationGroups | Create an LDAP Authorization Group object.
*DefaultApi* | [**create_msg_vpn_bridge**](docs/DefaultApi.md#create_msg_vpn_bridge) | **Post** /msgVpns/{msgVpnName}/bridges | Create a Bridge object.
*DefaultApi* | [**create_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#create_msg_vpn_bridge_remote_msg_vpn) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Create a Remote Message VPN object.
*DefaultApi* | [**create_msg_vpn_bridge_remote_subscription**](docs/DefaultApi.md#create_msg_vpn_bridge_remote_subscription) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Create a Remote Subscription object.
*DefaultApi* | [**create_msg_vpn_bridge_tls_trusted_common_name**](docs/DefaultApi.md#create_msg_vpn_bridge_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*DefaultApi* | [**create_msg_vpn_client_profile**](docs/DefaultApi.md#create_msg_vpn_client_profile) | **Post** /msgVpns/{msgVpnName}/clientProfiles | Create a Client Profile object.
*DefaultApi* | [**create_msg_vpn_client_username**](docs/DefaultApi.md#create_msg_vpn_client_username) | **Post** /msgVpns/{msgVpnName}/clientUsernames | Create a Client Username object.
*DefaultApi* | [**create_msg_vpn_distributed_cache**](docs/DefaultApi.md#create_msg_vpn_distributed_cache) | **Post** /msgVpns/{msgVpnName}/distributedCaches | Create a Distributed Cache object.
*DefaultApi* | [**create_msg_vpn_distributed_cache_cluster**](docs/DefaultApi.md#create_msg_vpn_distributed_cache_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Create a Cache Cluster object.
*DefaultApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/DefaultApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Create a Home Cache Cluster object.
*DefaultApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/DefaultApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Create a Topic Prefix object.
*DefaultApi* | [**create_msg_vpn_distributed_cache_cluster_instance**](docs/DefaultApi.md#create_msg_vpn_distributed_cache_cluster_instance) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Create a Cache Instance object.
*DefaultApi* | [**create_msg_vpn_distributed_cache_cluster_topic**](docs/DefaultApi.md#create_msg_vpn_distributed_cache_cluster_topic) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Create a Topic object.
*DefaultApi* | [**create_msg_vpn_dmr_bridge**](docs/DefaultApi.md#create_msg_vpn_dmr_bridge) | **Post** /msgVpns/{msgVpnName}/dmrBridges | Create a DMR Bridge object.
*DefaultApi* | [**create_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#create_msg_vpn_jndi_connection_factory) | **Post** /msgVpns/{msgVpnName}/jndiConnectionFactories | Create a JNDI Connection Factory object.
*DefaultApi* | [**create_msg_vpn_jndi_queue**](docs/DefaultApi.md#create_msg_vpn_jndi_queue) | **Post** /msgVpns/{msgVpnName}/jndiQueues | Create a JNDI Queue object.
*DefaultApi* | [**create_msg_vpn_jndi_topic**](docs/DefaultApi.md#create_msg_vpn_jndi_topic) | **Post** /msgVpns/{msgVpnName}/jndiTopics | Create a JNDI Topic object.
*DefaultApi* | [**create_msg_vpn_mqtt_retain_cache**](docs/DefaultApi.md#create_msg_vpn_mqtt_retain_cache) | **Post** /msgVpns/{msgVpnName}/mqttRetainCaches | Create an MQTT Retain Cache object.
*DefaultApi* | [**create_msg_vpn_mqtt_session**](docs/DefaultApi.md#create_msg_vpn_mqtt_session) | **Post** /msgVpns/{msgVpnName}/mqttSessions | Create an MQTT Session object.
*DefaultApi* | [**create_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#create_msg_vpn_mqtt_session_subscription) | **Post** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Create a Subscription object.
*DefaultApi* | [**create_msg_vpn_queue**](docs/DefaultApi.md#create_msg_vpn_queue) | **Post** /msgVpns/{msgVpnName}/queues | Create a Queue object.
*DefaultApi* | [**create_msg_vpn_queue_subscription**](docs/DefaultApi.md#create_msg_vpn_queue_subscription) | **Post** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Create a Subscription object.
*DefaultApi* | [**create_msg_vpn_replay_log**](docs/DefaultApi.md#create_msg_vpn_replay_log) | **Post** /msgVpns/{msgVpnName}/replayLogs | Create a Replay Log object.
*DefaultApi* | [**create_msg_vpn_replicated_topic**](docs/DefaultApi.md#create_msg_vpn_replicated_topic) | **Post** /msgVpns/{msgVpnName}/replicatedTopics | Create a Replicated Topic object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints | Create a REST Delivery Point object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point_queue_binding) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Create a Queue Binding object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point_rest_consumer) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Create a REST Consumer object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*DefaultApi* | [**create_msg_vpn_sequenced_topic**](docs/DefaultApi.md#create_msg_vpn_sequenced_topic) | **Post** /msgVpns/{msgVpnName}/sequencedTopics | Create a Sequenced Topic object.
*DefaultApi* | [**create_msg_vpn_topic_endpoint**](docs/DefaultApi.md#create_msg_vpn_topic_endpoint) | **Post** /msgVpns/{msgVpnName}/topicEndpoints | Create a Topic Endpoint object.
*DefaultApi* | [**delete_dmr_cluster**](docs/DefaultApi.md#delete_dmr_cluster) | **Delete** /dmrClusters/{dmrClusterName} | Delete a Cluster object.
*DefaultApi* | [**delete_dmr_cluster_link**](docs/DefaultApi.md#delete_dmr_cluster_link) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Delete a Link object.
*DefaultApi* | [**delete_dmr_cluster_link_remote_address**](docs/DefaultApi.md#delete_dmr_cluster_link_remote_address) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Delete a Remote Address object.
*DefaultApi* | [**delete_dmr_cluster_link_tls_trusted_common_name**](docs/DefaultApi.md#delete_dmr_cluster_link_tls_trusted_common_name) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*DefaultApi* | [**delete_msg_vpn**](docs/DefaultApi.md#delete_msg_vpn) | **Delete** /msgVpns/{msgVpnName} | Delete a Message VPN object.
*DefaultApi* | [**delete_msg_vpn_acl_profile**](docs/DefaultApi.md#delete_msg_vpn_acl_profile) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Delete an ACL Profile object.
*DefaultApi* | [**delete_msg_vpn_acl_profile_client_connect_exception**](docs/DefaultApi.md#delete_msg_vpn_acl_profile_client_connect_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Delete a Client Connect Exception object.
*DefaultApi* | [**delete_msg_vpn_acl_profile_publish_exception**](docs/DefaultApi.md#delete_msg_vpn_acl_profile_publish_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Delete a Publish Topic Exception object.
*DefaultApi* | [**delete_msg_vpn_acl_profile_subscribe_exception**](docs/DefaultApi.md#delete_msg_vpn_acl_profile_subscribe_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Delete a Subscribe Topic Exception object.
*DefaultApi* | [**delete_msg_vpn_authorization_group**](docs/DefaultApi.md#delete_msg_vpn_authorization_group) | **Delete** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Delete an LDAP Authorization Group object.
*DefaultApi* | [**delete_msg_vpn_bridge**](docs/DefaultApi.md#delete_msg_vpn_bridge) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Delete a Bridge object.
*DefaultApi* | [**delete_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#delete_msg_vpn_bridge_remote_msg_vpn) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Delete a Remote Message VPN object.
*DefaultApi* | [**delete_msg_vpn_bridge_remote_subscription**](docs/DefaultApi.md#delete_msg_vpn_bridge_remote_subscription) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Delete a Remote Subscription object.
*DefaultApi* | [**delete_msg_vpn_bridge_tls_trusted_common_name**](docs/DefaultApi.md#delete_msg_vpn_bridge_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*DefaultApi* | [**delete_msg_vpn_client_profile**](docs/DefaultApi.md#delete_msg_vpn_client_profile) | **Delete** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Delete a Client Profile object.
*DefaultApi* | [**delete_msg_vpn_client_username**](docs/DefaultApi.md#delete_msg_vpn_client_username) | **Delete** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Delete a Client Username object.
*DefaultApi* | [**delete_msg_vpn_distributed_cache**](docs/DefaultApi.md#delete_msg_vpn_distributed_cache) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Delete a Distributed Cache object.
*DefaultApi* | [**delete_msg_vpn_distributed_cache_cluster**](docs/DefaultApi.md#delete_msg_vpn_distributed_cache_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Delete a Cache Cluster object.
*DefaultApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/DefaultApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Delete a Home Cache Cluster object.
*DefaultApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/DefaultApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Delete a Topic Prefix object.
*DefaultApi* | [**delete_msg_vpn_distributed_cache_cluster_instance**](docs/DefaultApi.md#delete_msg_vpn_distributed_cache_cluster_instance) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Delete a Cache Instance object.
*DefaultApi* | [**delete_msg_vpn_distributed_cache_cluster_topic**](docs/DefaultApi.md#delete_msg_vpn_distributed_cache_cluster_topic) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Delete a Topic object.
*DefaultApi* | [**delete_msg_vpn_dmr_bridge**](docs/DefaultApi.md#delete_msg_vpn_dmr_bridge) | **Delete** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Delete a DMR Bridge object.
*DefaultApi* | [**delete_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#delete_msg_vpn_jndi_connection_factory) | **Delete** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Delete a JNDI Connection Factory object.
*DefaultApi* | [**delete_msg_vpn_jndi_queue**](docs/DefaultApi.md#delete_msg_vpn_jndi_queue) | **Delete** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Delete a JNDI Queue object.
*DefaultApi* | [**delete_msg_vpn_jndi_topic**](docs/DefaultApi.md#delete_msg_vpn_jndi_topic) | **Delete** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Delete a JNDI Topic object.
*DefaultApi* | [**delete_msg_vpn_mqtt_retain_cache**](docs/DefaultApi.md#delete_msg_vpn_mqtt_retain_cache) | **Delete** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Delete an MQTT Retain Cache object.
*DefaultApi* | [**delete_msg_vpn_mqtt_session**](docs/DefaultApi.md#delete_msg_vpn_mqtt_session) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Delete an MQTT Session object.
*DefaultApi* | [**delete_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#delete_msg_vpn_mqtt_session_subscription) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Delete a Subscription object.
*DefaultApi* | [**delete_msg_vpn_queue**](docs/DefaultApi.md#delete_msg_vpn_queue) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName} | Delete a Queue object.
*DefaultApi* | [**delete_msg_vpn_queue_subscription**](docs/DefaultApi.md#delete_msg_vpn_queue_subscription) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Delete a Subscription object.
*DefaultApi* | [**delete_msg_vpn_replay_log**](docs/DefaultApi.md#delete_msg_vpn_replay_log) | **Delete** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Delete a Replay Log object.
*DefaultApi* | [**delete_msg_vpn_replicated_topic**](docs/DefaultApi.md#delete_msg_vpn_replicated_topic) | **Delete** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Delete a Replicated Topic object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Delete a REST Delivery Point object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point_queue_binding) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Delete a Queue Binding object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Delete a REST Consumer object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*DefaultApi* | [**delete_msg_vpn_sequenced_topic**](docs/DefaultApi.md#delete_msg_vpn_sequenced_topic) | **Delete** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Delete a Sequenced Topic object.
*DefaultApi* | [**delete_msg_vpn_topic_endpoint**](docs/DefaultApi.md#delete_msg_vpn_topic_endpoint) | **Delete** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Delete a Topic Endpoint object.
*DefaultApi* | [**get_about_user_msg_vpn**](docs/DefaultApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
*DefaultApi* | [**get_about_user_msg_vpns**](docs/DefaultApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
*DefaultApi* | [**get_dmr_cluster**](docs/DefaultApi.md#get_dmr_cluster) | **Get** /dmrClusters/{dmrClusterName} | Get a Cluster object.
*DefaultApi* | [**get_dmr_cluster_link**](docs/DefaultApi.md#get_dmr_cluster_link) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Get a Link object.
*DefaultApi* | [**get_dmr_cluster_link_remote_address**](docs/DefaultApi.md#get_dmr_cluster_link_remote_address) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Get a Remote Address object.
*DefaultApi* | [**get_dmr_cluster_link_remote_addresses**](docs/DefaultApi.md#get_dmr_cluster_link_remote_addresses) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Get a list of Remote Address objects.
*DefaultApi* | [**get_dmr_cluster_link_tls_trusted_common_name**](docs/DefaultApi.md#get_dmr_cluster_link_tls_trusted_common_name) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*DefaultApi* | [**get_dmr_cluster_link_tls_trusted_common_names**](docs/DefaultApi.md#get_dmr_cluster_link_tls_trusted_common_names) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*DefaultApi* | [**get_dmr_cluster_links**](docs/DefaultApi.md#get_dmr_cluster_links) | **Get** /dmrClusters/{dmrClusterName}/links | Get a list of Link objects.
*DefaultApi* | [**get_msg_vpn**](docs/DefaultApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Get a Message VPN object.
*DefaultApi* | [**get_msg_vpn_acl_profile**](docs/DefaultApi.md#get_msg_vpn_acl_profile) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Get an ACL Profile object.
*DefaultApi* | [**get_msg_vpn_acl_profile_client_connect_exception**](docs/DefaultApi.md#get_msg_vpn_acl_profile_client_connect_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Get a Client Connect Exception object.
*DefaultApi* | [**get_msg_vpn_acl_profile_client_connect_exceptions**](docs/DefaultApi.md#get_msg_vpn_acl_profile_client_connect_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Get a list of Client Connect Exception objects.
*DefaultApi* | [**get_msg_vpn_acl_profile_publish_exception**](docs/DefaultApi.md#get_msg_vpn_acl_profile_publish_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Get a Publish Topic Exception object.
*DefaultApi* | [**get_msg_vpn_acl_profile_publish_exceptions**](docs/DefaultApi.md#get_msg_vpn_acl_profile_publish_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Get a list of Publish Topic Exception objects.
*DefaultApi* | [**get_msg_vpn_acl_profile_subscribe_exception**](docs/DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Get a Subscribe Topic Exception object.
*DefaultApi* | [**get_msg_vpn_acl_profile_subscribe_exceptions**](docs/DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Get a list of Subscribe Topic Exception objects.
*DefaultApi* | [**get_msg_vpn_authorization_group**](docs/DefaultApi.md#get_msg_vpn_authorization_group) | **Get** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Get an LDAP Authorization Group object.
*DefaultApi* | [**get_msg_vpn_bridge**](docs/DefaultApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
*DefaultApi* | [**get_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_msg_vpn) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Get a Remote Message VPN object.
*DefaultApi* | [**get_msg_vpn_bridge_remote_msg_vpns**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_msg_vpns) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Get a list of Remote Message VPN objects.
*DefaultApi* | [**get_msg_vpn_bridge_remote_subscription**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Get a Remote Subscription object.
*DefaultApi* | [**get_msg_vpn_bridge_remote_subscriptions**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Get a list of Remote Subscription objects.
*DefaultApi* | [**get_msg_vpn_bridge_tls_trusted_common_name**](docs/DefaultApi.md#get_msg_vpn_bridge_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*DefaultApi* | [**get_msg_vpn_bridge_tls_trusted_common_names**](docs/DefaultApi.md#get_msg_vpn_bridge_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*DefaultApi* | [**get_msg_vpn_client_profile**](docs/DefaultApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Get a Client Profile object.
*DefaultApi* | [**get_msg_vpn_client_username**](docs/DefaultApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Get a Client Username object.
*DefaultApi* | [**get_msg_vpn_distributed_cache**](docs/DefaultApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Get a Home Cache Cluster object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Get a Topic Prefix object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Get a list of Topic Prefix objects.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Get a list of Home Cache Cluster objects.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_instance**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_instances**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_topic**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_topic) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Get a Topic object.
*DefaultApi* | [**get_msg_vpn_distributed_cache_cluster_topics**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_cluster_topics) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Get a list of Topic objects.
*DefaultApi* | [**get_msg_vpn_distributed_cache_clusters**](docs/DefaultApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
*DefaultApi* | [**get_msg_vpn_dmr_bridge**](docs/DefaultApi.md#get_msg_vpn_dmr_bridge) | **Get** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Get a DMR Bridge object.
*DefaultApi* | [**get_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#get_msg_vpn_jndi_connection_factory) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Get a JNDI Connection Factory object.
*DefaultApi* | [**get_msg_vpn_jndi_queue**](docs/DefaultApi.md#get_msg_vpn_jndi_queue) | **Get** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Get a JNDI Queue object.
*DefaultApi* | [**get_msg_vpn_jndi_topic**](docs/DefaultApi.md#get_msg_vpn_jndi_topic) | **Get** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Get a JNDI Topic object.
*DefaultApi* | [**get_msg_vpn_mqtt_retain_cache**](docs/DefaultApi.md#get_msg_vpn_mqtt_retain_cache) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Get an MQTT Retain Cache object.
*DefaultApi* | [**get_msg_vpn_mqtt_session**](docs/DefaultApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
*DefaultApi* | [**get_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#get_msg_vpn_mqtt_session_subscription) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Get a Subscription object.
*DefaultApi* | [**get_msg_vpn_mqtt_session_subscriptions**](docs/DefaultApi.md#get_msg_vpn_mqtt_session_subscriptions) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Get a list of Subscription objects.
*DefaultApi* | [**get_msg_vpn_queue**](docs/DefaultApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
*DefaultApi* | [**get_msg_vpn_queue_subscription**](docs/DefaultApi.md#get_msg_vpn_queue_subscription) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Get a Subscription object.
*DefaultApi* | [**get_msg_vpn_queue_subscriptions**](docs/DefaultApi.md#get_msg_vpn_queue_subscriptions) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Get a list of Subscription objects.
*DefaultApi* | [**get_msg_vpn_replay_log**](docs/DefaultApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
*DefaultApi* | [**get_msg_vpn_replicated_topic**](docs/DefaultApi.md#get_msg_vpn_replicated_topic) | **Get** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Get a Replicated Topic object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Get a Queue Binding object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_queue_bindings**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Get a list of Queue Binding objects.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumers**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
*DefaultApi* | [**get_msg_vpn_sequenced_topic**](docs/DefaultApi.md#get_msg_vpn_sequenced_topic) | **Get** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Get a Sequenced Topic object.
*DefaultApi* | [**get_msg_vpn_sequenced_topics**](docs/DefaultApi.md#get_msg_vpn_sequenced_topics) | **Get** /msgVpns/{msgVpnName}/sequencedTopics | Get a list of Sequenced Topic objects.
*DefaultApi* | [**get_msg_vpn_topic_endpoint**](docs/DefaultApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
*DefaultApi* | [**replace_dmr_cluster**](docs/DefaultApi.md#replace_dmr_cluster) | **Put** /dmrClusters/{dmrClusterName} | Replace a Cluster object.
*DefaultApi* | [**replace_dmr_cluster_link**](docs/DefaultApi.md#replace_dmr_cluster_link) | **Put** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Replace a Link object.
*DefaultApi* | [**replace_msg_vpn**](docs/DefaultApi.md#replace_msg_vpn) | **Put** /msgVpns/{msgVpnName} | Replace a Message VPN object.
*DefaultApi* | [**replace_msg_vpn_acl_profile**](docs/DefaultApi.md#replace_msg_vpn_acl_profile) | **Put** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Replace an ACL Profile object.
*DefaultApi* | [**replace_msg_vpn_authorization_group**](docs/DefaultApi.md#replace_msg_vpn_authorization_group) | **Put** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Replace an LDAP Authorization Group object.
*DefaultApi* | [**replace_msg_vpn_bridge**](docs/DefaultApi.md#replace_msg_vpn_bridge) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Replace a Bridge object.
*DefaultApi* | [**replace_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#replace_msg_vpn_bridge_remote_msg_vpn) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Replace a Remote Message VPN object.
*DefaultApi* | [**replace_msg_vpn_client_profile**](docs/DefaultApi.md#replace_msg_vpn_client_profile) | **Put** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Replace a Client Profile object.
*DefaultApi* | [**replace_msg_vpn_client_username**](docs/DefaultApi.md#replace_msg_vpn_client_username) | **Put** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Replace a Client Username object.
*DefaultApi* | [**replace_msg_vpn_distributed_cache**](docs/DefaultApi.md#replace_msg_vpn_distributed_cache) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Replace a Distributed Cache object.
*DefaultApi* | [**replace_msg_vpn_distributed_cache_cluster**](docs/DefaultApi.md#replace_msg_vpn_distributed_cache_cluster) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Replace a Cache Cluster object.
*DefaultApi* | [**replace_msg_vpn_distributed_cache_cluster_instance**](docs/DefaultApi.md#replace_msg_vpn_distributed_cache_cluster_instance) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Replace a Cache Instance object.
*DefaultApi* | [**replace_msg_vpn_dmr_bridge**](docs/DefaultApi.md#replace_msg_vpn_dmr_bridge) | **Put** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Replace a DMR Bridge object.
*DefaultApi* | [**replace_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#replace_msg_vpn_jndi_connection_factory) | **Put** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Replace a JNDI Connection Factory object.
*DefaultApi* | [**replace_msg_vpn_jndi_queue**](docs/DefaultApi.md#replace_msg_vpn_jndi_queue) | **Put** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Replace a JNDI Queue object.
*DefaultApi* | [**replace_msg_vpn_jndi_topic**](docs/DefaultApi.md#replace_msg_vpn_jndi_topic) | **Put** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Replace a JNDI Topic object.
*DefaultApi* | [**replace_msg_vpn_mqtt_retain_cache**](docs/DefaultApi.md#replace_msg_vpn_mqtt_retain_cache) | **Put** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Replace an MQTT Retain Cache object.
*DefaultApi* | [**replace_msg_vpn_mqtt_session**](docs/DefaultApi.md#replace_msg_vpn_mqtt_session) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Replace an MQTT Session object.
*DefaultApi* | [**replace_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#replace_msg_vpn_mqtt_session_subscription) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Replace a Subscription object.
*DefaultApi* | [**replace_msg_vpn_queue**](docs/DefaultApi.md#replace_msg_vpn_queue) | **Put** /msgVpns/{msgVpnName}/queues/{queueName} | Replace a Queue object.
*DefaultApi* | [**replace_msg_vpn_replay_log**](docs/DefaultApi.md#replace_msg_vpn_replay_log) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Replace a Replay Log object.
*DefaultApi* | [**replace_msg_vpn_replicated_topic**](docs/DefaultApi.md#replace_msg_vpn_replicated_topic) | **Put** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Replace a Replicated Topic object.
*DefaultApi* | [**replace_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#replace_msg_vpn_rest_delivery_point) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Replace a REST Delivery Point object.
*DefaultApi* | [**replace_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#replace_msg_vpn_rest_delivery_point_queue_binding) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Replace a Queue Binding object.
*DefaultApi* | [**replace_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#replace_msg_vpn_rest_delivery_point_rest_consumer) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Replace a REST Consumer object.
*DefaultApi* | [**replace_msg_vpn_topic_endpoint**](docs/DefaultApi.md#replace_msg_vpn_topic_endpoint) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Replace a Topic Endpoint object.
*DefaultApi* | [**update_dmr_cluster**](docs/DefaultApi.md#update_dmr_cluster) | **Patch** /dmrClusters/{dmrClusterName} | Update a Cluster object.
*DefaultApi* | [**update_dmr_cluster_link**](docs/DefaultApi.md#update_dmr_cluster_link) | **Patch** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Update a Link object.
*DefaultApi* | [**update_msg_vpn**](docs/DefaultApi.md#update_msg_vpn) | **Patch** /msgVpns/{msgVpnName} | Update a Message VPN object.
*DefaultApi* | [**update_msg_vpn_acl_profile**](docs/DefaultApi.md#update_msg_vpn_acl_profile) | **Patch** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Update an ACL Profile object.
*DefaultApi* | [**update_msg_vpn_authorization_group**](docs/DefaultApi.md#update_msg_vpn_authorization_group) | **Patch** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Update an LDAP Authorization Group object.
*DefaultApi* | [**update_msg_vpn_bridge**](docs/DefaultApi.md#update_msg_vpn_bridge) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Update a Bridge object.
*DefaultApi* | [**update_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#update_msg_vpn_bridge_remote_msg_vpn) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Update a Remote Message VPN object.
*DefaultApi* | [**update_msg_vpn_client_profile**](docs/DefaultApi.md#update_msg_vpn_client_profile) | **Patch** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Update a Client Profile object.
*DefaultApi* | [**update_msg_vpn_client_username**](docs/DefaultApi.md#update_msg_vpn_client_username) | **Patch** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Update a Client Username object.
*DefaultApi* | [**update_msg_vpn_distributed_cache**](docs/DefaultApi.md#update_msg_vpn_distributed_cache) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Update a Distributed Cache object.
*DefaultApi* | [**update_msg_vpn_distributed_cache_cluster**](docs/DefaultApi.md#update_msg_vpn_distributed_cache_cluster) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Update a Cache Cluster object.
*DefaultApi* | [**update_msg_vpn_distributed_cache_cluster_instance**](docs/DefaultApi.md#update_msg_vpn_distributed_cache_cluster_instance) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Update a Cache Instance object.
*DefaultApi* | [**update_msg_vpn_dmr_bridge**](docs/DefaultApi.md#update_msg_vpn_dmr_bridge) | **Patch** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Update a DMR Bridge object.
*DefaultApi* | [**update_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#update_msg_vpn_jndi_connection_factory) | **Patch** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Update a JNDI Connection Factory object.
*DefaultApi* | [**update_msg_vpn_jndi_queue**](docs/DefaultApi.md#update_msg_vpn_jndi_queue) | **Patch** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Update a JNDI Queue object.
*DefaultApi* | [**update_msg_vpn_jndi_topic**](docs/DefaultApi.md#update_msg_vpn_jndi_topic) | **Patch** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Update a JNDI Topic object.
*DefaultApi* | [**update_msg_vpn_mqtt_retain_cache**](docs/DefaultApi.md#update_msg_vpn_mqtt_retain_cache) | **Patch** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Update an MQTT Retain Cache object.
*DefaultApi* | [**update_msg_vpn_mqtt_session**](docs/DefaultApi.md#update_msg_vpn_mqtt_session) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Update an MQTT Session object.
*DefaultApi* | [**update_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#update_msg_vpn_mqtt_session_subscription) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Update a Subscription object.
*DefaultApi* | [**update_msg_vpn_queue**](docs/DefaultApi.md#update_msg_vpn_queue) | **Patch** /msgVpns/{msgVpnName}/queues/{queueName} | Update a Queue object.
*DefaultApi* | [**update_msg_vpn_replay_log**](docs/DefaultApi.md#update_msg_vpn_replay_log) | **Patch** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Update a Replay Log object.
*DefaultApi* | [**update_msg_vpn_replicated_topic**](docs/DefaultApi.md#update_msg_vpn_replicated_topic) | **Patch** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Update a Replicated Topic object.
*DefaultApi* | [**update_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#update_msg_vpn_rest_delivery_point) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Update a REST Delivery Point object.
*DefaultApi* | [**update_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#update_msg_vpn_rest_delivery_point_queue_binding) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Update a Queue Binding object.
*DefaultApi* | [**update_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#update_msg_vpn_rest_delivery_point_rest_consumer) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Update a REST Consumer object.
*DefaultApi* | [**update_msg_vpn_topic_endpoint**](docs/DefaultApi.md#update_msg_vpn_topic_endpoint) | **Patch** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Update a Topic Endpoint object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_caches**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*DmrBridgeApi* | [**get_msg_vpn_dmr_bridges**](docs/DmrBridgeApi.md#get_msg_vpn_dmr_bridges) | **Get** /msgVpns/{msgVpnName}/dmrBridges | Get a list of DMR Bridge objects.
*DmrClusterApi* | [**get_dmr_clusters**](docs/DmrClusterApi.md#get_dmr_clusters) | **Get** /dmrClusters | Get a list of Cluster objects.
*JndiApi* | [**get_msg_vpn_jndi_connection_factories**](docs/JndiApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Get a list of JNDI Connection Factory objects.
*JndiApi* | [**get_msg_vpn_jndi_queues**](docs/JndiApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Get a list of JNDI Queue objects.
*JndiApi* | [**get_msg_vpn_jndi_topics**](docs/JndiApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Get a list of JNDI Topic objects.
*MqttRetainCacheApi* | [**get_msg_vpn_mqtt_retain_caches**](docs/MqttRetainCacheApi.md#get_msg_vpn_mqtt_retain_caches) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches | Get a list of MQTT Retain Cache objects.
*MqttSessionApi* | [**get_msg_vpn_mqtt_sessions**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profiles**](docs/MsgVpnApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Get a list of ACL Profile objects.
*MsgVpnApi* | [**get_msg_vpn_authorization_groups**](docs/MsgVpnApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Get a list of LDAP Authorization Group objects.
*MsgVpnApi* | [**get_msg_vpn_bridges**](docs/MsgVpnApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*MsgVpnApi* | [**get_msg_vpn_client_profiles**](docs/MsgVpnApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Get a list of Client Profile objects.
*MsgVpnApi* | [**get_msg_vpn_client_usernames**](docs/MsgVpnApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Get a list of Client Username objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_caches**](docs/MsgVpnApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*MsgVpnApi* | [**get_msg_vpn_dmr_bridges**](docs/MsgVpnApi.md#get_msg_vpn_dmr_bridges) | **Get** /msgVpns/{msgVpnName}/dmrBridges | Get a list of DMR Bridge objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_connection_factories**](docs/MsgVpnApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Get a list of JNDI Connection Factory objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_queues**](docs/MsgVpnApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Get a list of JNDI Queue objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_topics**](docs/MsgVpnApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Get a list of JNDI Topic objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_retain_caches**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_retain_caches) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches | Get a list of MQTT Retain Cache objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_sessions**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_queues**](docs/MsgVpnApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*MsgVpnApi* | [**get_msg_vpn_replay_logs**](docs/MsgVpnApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*MsgVpnApi* | [**get_msg_vpn_replicated_topics**](docs/MsgVpnApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Get a list of Replicated Topic objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_points**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoints**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.
*MsgVpnApi* | [**get_msg_vpns**](docs/MsgVpnApi.md#get_msg_vpns) | **Get** /msgVpns | Get a list of Message VPN objects.
*QueueApi* | [**get_msg_vpn_queues**](docs/QueueApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*ReplayLogApi* | [**get_msg_vpn_replay_logs**](docs/ReplayLogApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*ReplicatedTopicApi* | [**get_msg_vpn_replicated_topics**](docs/ReplicatedTopicApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Get a list of Replicated Topic objects.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_points**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*SystemInformationApi* | [**get_system_information**](docs/SystemInformationApi.md#get_system_information) | **Get** /systemInformation | Get SEMP API version and platform information.
*TopicEndpointApi* | [**get_msg_vpn_topic_endpoints**](docs/TopicEndpointApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.


## Documentation For Models

 - [AboutApi](docs/AboutApi.md)
 - [AboutApiLinks](docs/AboutApiLinks.md)
 - [AboutApiResponse](docs/AboutApiResponse.md)
 - [AboutUser](docs/AboutUser.md)
 - [AboutUserLinks](docs/AboutUserLinks.md)
 - [AboutUserMsgVpn](docs/AboutUserMsgVpn.md)
 - [AboutUserMsgVpnLinks](docs/AboutUserMsgVpnLinks.md)
 - [AboutUserMsgVpnResponse](docs/AboutUserMsgVpnResponse.md)
 - [AboutUserMsgVpnsResponse](docs/AboutUserMsgVpnsResponse.md)
 - [AboutUserResponse](docs/AboutUserResponse.md)
 - [DmrCluster](docs/DmrCluster.md)
 - [DmrClusterLink](docs/DmrClusterLink.md)
 - [DmrClusterLinkLinks](docs/DmrClusterLinkLinks.md)
 - [DmrClusterLinkRemoteAddress](docs/DmrClusterLinkRemoteAddress.md)
 - [DmrClusterLinkRemoteAddressLinks](docs/DmrClusterLinkRemoteAddressLinks.md)
 - [DmrClusterLinkRemoteAddressResponse](docs/DmrClusterLinkRemoteAddressResponse.md)
 - [DmrClusterLinkRemoteAddressesResponse](docs/DmrClusterLinkRemoteAddressesResponse.md)
 - [DmrClusterLinkResponse](docs/DmrClusterLinkResponse.md)
 - [DmrClusterLinkTlsTrustedCommonName](docs/DmrClusterLinkTlsTrustedCommonName.md)
 - [DmrClusterLinkTlsTrustedCommonNameLinks](docs/DmrClusterLinkTlsTrustedCommonNameLinks.md)
 - [DmrClusterLinkTlsTrustedCommonNameResponse](docs/DmrClusterLinkTlsTrustedCommonNameResponse.md)
 - [DmrClusterLinkTlsTrustedCommonNamesResponse](docs/DmrClusterLinkTlsTrustedCommonNamesResponse.md)
 - [DmrClusterLinks](docs/DmrClusterLinks.md)
 - [DmrClusterLinksResponse](docs/DmrClusterLinksResponse.md)
 - [DmrClusterResponse](docs/DmrClusterResponse.md)
 - [DmrClustersResponse](docs/DmrClustersResponse.md)
 - [EventThreshold](docs/EventThreshold.md)
 - [EventThresholdByPercent](docs/EventThresholdByPercent.md)
 - [EventThresholdByValue](docs/EventThresholdByValue.md)
 - [MsgVpn](docs/MsgVpn.md)
 - [MsgVpnAclProfile](docs/MsgVpnAclProfile.md)
 - [MsgVpnAclProfileClientConnectException](docs/MsgVpnAclProfileClientConnectException.md)
 - [MsgVpnAclProfileClientConnectExceptionLinks](docs/MsgVpnAclProfileClientConnectExceptionLinks.md)
 - [MsgVpnAclProfileClientConnectExceptionResponse](docs/MsgVpnAclProfileClientConnectExceptionResponse.md)
 - [MsgVpnAclProfileClientConnectExceptionsResponse](docs/MsgVpnAclProfileClientConnectExceptionsResponse.md)
 - [MsgVpnAclProfileLinks](docs/MsgVpnAclProfileLinks.md)
 - [MsgVpnAclProfilePublishException](docs/MsgVpnAclProfilePublishException.md)
 - [MsgVpnAclProfilePublishExceptionLinks](docs/MsgVpnAclProfilePublishExceptionLinks.md)
 - [MsgVpnAclProfilePublishExceptionResponse](docs/MsgVpnAclProfilePublishExceptionResponse.md)
 - [MsgVpnAclProfilePublishExceptionsResponse](docs/MsgVpnAclProfilePublishExceptionsResponse.md)
 - [MsgVpnAclProfileResponse](docs/MsgVpnAclProfileResponse.md)
 - [MsgVpnAclProfileSubscribeException](docs/MsgVpnAclProfileSubscribeException.md)
 - [MsgVpnAclProfileSubscribeExceptionLinks](docs/MsgVpnAclProfileSubscribeExceptionLinks.md)
 - [MsgVpnAclProfileSubscribeExceptionResponse](docs/MsgVpnAclProfileSubscribeExceptionResponse.md)
 - [MsgVpnAclProfileSubscribeExceptionsResponse](docs/MsgVpnAclProfileSubscribeExceptionsResponse.md)
 - [MsgVpnAclProfilesResponse](docs/MsgVpnAclProfilesResponse.md)
 - [MsgVpnAuthorizationGroup](docs/MsgVpnAuthorizationGroup.md)
 - [MsgVpnAuthorizationGroupLinks](docs/MsgVpnAuthorizationGroupLinks.md)
 - [MsgVpnAuthorizationGroupResponse](docs/MsgVpnAuthorizationGroupResponse.md)
 - [MsgVpnAuthorizationGroupsResponse](docs/MsgVpnAuthorizationGroupsResponse.md)
 - [MsgVpnBridge](docs/MsgVpnBridge.md)
 - [MsgVpnBridgeLinks](docs/MsgVpnBridgeLinks.md)
 - [MsgVpnBridgeRemoteMsgVpn](docs/MsgVpnBridgeRemoteMsgVpn.md)
 - [MsgVpnBridgeRemoteMsgVpnLinks](docs/MsgVpnBridgeRemoteMsgVpnLinks.md)
 - [MsgVpnBridgeRemoteMsgVpnResponse](docs/MsgVpnBridgeRemoteMsgVpnResponse.md)
 - [MsgVpnBridgeRemoteMsgVpnsResponse](docs/MsgVpnBridgeRemoteMsgVpnsResponse.md)
 - [MsgVpnBridgeRemoteSubscription](docs/MsgVpnBridgeRemoteSubscription.md)
 - [MsgVpnBridgeRemoteSubscriptionLinks](docs/MsgVpnBridgeRemoteSubscriptionLinks.md)
 - [MsgVpnBridgeRemoteSubscriptionResponse](docs/MsgVpnBridgeRemoteSubscriptionResponse.md)
 - [MsgVpnBridgeRemoteSubscriptionsResponse](docs/MsgVpnBridgeRemoteSubscriptionsResponse.md)
 - [MsgVpnBridgeResponse](docs/MsgVpnBridgeResponse.md)
 - [MsgVpnBridgeTlsTrustedCommonName](docs/MsgVpnBridgeTlsTrustedCommonName.md)
 - [MsgVpnBridgeTlsTrustedCommonNameLinks](docs/MsgVpnBridgeTlsTrustedCommonNameLinks.md)
 - [MsgVpnBridgeTlsTrustedCommonNameResponse](docs/MsgVpnBridgeTlsTrustedCommonNameResponse.md)
 - [MsgVpnBridgeTlsTrustedCommonNamesResponse](docs/MsgVpnBridgeTlsTrustedCommonNamesResponse.md)
 - [MsgVpnBridgesResponse](docs/MsgVpnBridgesResponse.md)
 - [MsgVpnClientProfile](docs/MsgVpnClientProfile.md)
 - [MsgVpnClientProfileLinks](docs/MsgVpnClientProfileLinks.md)
 - [MsgVpnClientProfileResponse](docs/MsgVpnClientProfileResponse.md)
 - [MsgVpnClientProfilesResponse](docs/MsgVpnClientProfilesResponse.md)
 - [MsgVpnClientUsername](docs/MsgVpnClientUsername.md)
 - [MsgVpnClientUsernameLinks](docs/MsgVpnClientUsernameLinks.md)
 - [MsgVpnClientUsernameResponse](docs/MsgVpnClientUsernameResponse.md)
 - [MsgVpnClientUsernamesResponse](docs/MsgVpnClientUsernamesResponse.md)
 - [MsgVpnDistributedCache](docs/MsgVpnDistributedCache.md)
 - [MsgVpnDistributedCacheCluster](docs/MsgVpnDistributedCacheCluster.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeCluster](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeCluster.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClusterLinks](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClusterLinks.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefix](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefix.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixLinks](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixLinks.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse.md)
 - [MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse](docs/MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse.md)
 - [MsgVpnDistributedCacheClusterInstance](docs/MsgVpnDistributedCacheClusterInstance.md)
 - [MsgVpnDistributedCacheClusterInstanceLinks](docs/MsgVpnDistributedCacheClusterInstanceLinks.md)
 - [MsgVpnDistributedCacheClusterInstanceResponse](docs/MsgVpnDistributedCacheClusterInstanceResponse.md)
 - [MsgVpnDistributedCacheClusterInstancesResponse](docs/MsgVpnDistributedCacheClusterInstancesResponse.md)
 - [MsgVpnDistributedCacheClusterLinks](docs/MsgVpnDistributedCacheClusterLinks.md)
 - [MsgVpnDistributedCacheClusterResponse](docs/MsgVpnDistributedCacheClusterResponse.md)
 - [MsgVpnDistributedCacheClusterTopic](docs/MsgVpnDistributedCacheClusterTopic.md)
 - [MsgVpnDistributedCacheClusterTopicLinks](docs/MsgVpnDistributedCacheClusterTopicLinks.md)
 - [MsgVpnDistributedCacheClusterTopicResponse](docs/MsgVpnDistributedCacheClusterTopicResponse.md)
 - [MsgVpnDistributedCacheClusterTopicsResponse](docs/MsgVpnDistributedCacheClusterTopicsResponse.md)
 - [MsgVpnDistributedCacheClustersResponse](docs/MsgVpnDistributedCacheClustersResponse.md)
 - [MsgVpnDistributedCacheLinks](docs/MsgVpnDistributedCacheLinks.md)
 - [MsgVpnDistributedCacheResponse](docs/MsgVpnDistributedCacheResponse.md)
 - [MsgVpnDistributedCachesResponse](docs/MsgVpnDistributedCachesResponse.md)
 - [MsgVpnDmrBridge](docs/MsgVpnDmrBridge.md)
 - [MsgVpnDmrBridgeLinks](docs/MsgVpnDmrBridgeLinks.md)
 - [MsgVpnDmrBridgeResponse](docs/MsgVpnDmrBridgeResponse.md)
 - [MsgVpnDmrBridgesResponse](docs/MsgVpnDmrBridgesResponse.md)
 - [MsgVpnJndiConnectionFactoriesResponse](docs/MsgVpnJndiConnectionFactoriesResponse.md)
 - [MsgVpnJndiConnectionFactory](docs/MsgVpnJndiConnectionFactory.md)
 - [MsgVpnJndiConnectionFactoryLinks](docs/MsgVpnJndiConnectionFactoryLinks.md)
 - [MsgVpnJndiConnectionFactoryResponse](docs/MsgVpnJndiConnectionFactoryResponse.md)
 - [MsgVpnJndiQueue](docs/MsgVpnJndiQueue.md)
 - [MsgVpnJndiQueueLinks](docs/MsgVpnJndiQueueLinks.md)
 - [MsgVpnJndiQueueResponse](docs/MsgVpnJndiQueueResponse.md)
 - [MsgVpnJndiQueuesResponse](docs/MsgVpnJndiQueuesResponse.md)
 - [MsgVpnJndiTopic](docs/MsgVpnJndiTopic.md)
 - [MsgVpnJndiTopicLinks](docs/MsgVpnJndiTopicLinks.md)
 - [MsgVpnJndiTopicResponse](docs/MsgVpnJndiTopicResponse.md)
 - [MsgVpnJndiTopicsResponse](docs/MsgVpnJndiTopicsResponse.md)
 - [MsgVpnLinks](docs/MsgVpnLinks.md)
 - [MsgVpnMqttRetainCache](docs/MsgVpnMqttRetainCache.md)
 - [MsgVpnMqttRetainCacheLinks](docs/MsgVpnMqttRetainCacheLinks.md)
 - [MsgVpnMqttRetainCacheResponse](docs/MsgVpnMqttRetainCacheResponse.md)
 - [MsgVpnMqttRetainCachesResponse](docs/MsgVpnMqttRetainCachesResponse.md)
 - [MsgVpnMqttSession](docs/MsgVpnMqttSession.md)
 - [MsgVpnMqttSessionLinks](docs/MsgVpnMqttSessionLinks.md)
 - [MsgVpnMqttSessionResponse](docs/MsgVpnMqttSessionResponse.md)
 - [MsgVpnMqttSessionSubscription](docs/MsgVpnMqttSessionSubscription.md)
 - [MsgVpnMqttSessionSubscriptionLinks](docs/MsgVpnMqttSessionSubscriptionLinks.md)
 - [MsgVpnMqttSessionSubscriptionResponse](docs/MsgVpnMqttSessionSubscriptionResponse.md)
 - [MsgVpnMqttSessionSubscriptionsResponse](docs/MsgVpnMqttSessionSubscriptionsResponse.md)
 - [MsgVpnMqttSessionsResponse](docs/MsgVpnMqttSessionsResponse.md)
 - [MsgVpnQueue](docs/MsgVpnQueue.md)
 - [MsgVpnQueueLinks](docs/MsgVpnQueueLinks.md)
 - [MsgVpnQueueResponse](docs/MsgVpnQueueResponse.md)
 - [MsgVpnQueueSubscription](docs/MsgVpnQueueSubscription.md)
 - [MsgVpnQueueSubscriptionLinks](docs/MsgVpnQueueSubscriptionLinks.md)
 - [MsgVpnQueueSubscriptionResponse](docs/MsgVpnQueueSubscriptionResponse.md)
 - [MsgVpnQueueSubscriptionsResponse](docs/MsgVpnQueueSubscriptionsResponse.md)
 - [MsgVpnQueuesResponse](docs/MsgVpnQueuesResponse.md)
 - [MsgVpnReplayLog](docs/MsgVpnReplayLog.md)
 - [MsgVpnReplayLogLinks](docs/MsgVpnReplayLogLinks.md)
 - [MsgVpnReplayLogResponse](docs/MsgVpnReplayLogResponse.md)
 - [MsgVpnReplayLogsResponse](docs/MsgVpnReplayLogsResponse.md)
 - [MsgVpnReplicatedTopic](docs/MsgVpnReplicatedTopic.md)
 - [MsgVpnReplicatedTopicLinks](docs/MsgVpnReplicatedTopicLinks.md)
 - [MsgVpnReplicatedTopicResponse](docs/MsgVpnReplicatedTopicResponse.md)
 - [MsgVpnReplicatedTopicsResponse](docs/MsgVpnReplicatedTopicsResponse.md)
 - [MsgVpnResponse](docs/MsgVpnResponse.md)
 - [MsgVpnRestDeliveryPoint](docs/MsgVpnRestDeliveryPoint.md)
 - [MsgVpnRestDeliveryPointLinks](docs/MsgVpnRestDeliveryPointLinks.md)
 - [MsgVpnRestDeliveryPointQueueBinding](docs/MsgVpnRestDeliveryPointQueueBinding.md)
 - [MsgVpnRestDeliveryPointQueueBindingLinks](docs/MsgVpnRestDeliveryPointQueueBindingLinks.md)
 - [MsgVpnRestDeliveryPointQueueBindingResponse](docs/MsgVpnRestDeliveryPointQueueBindingResponse.md)
 - [MsgVpnRestDeliveryPointQueueBindingsResponse](docs/MsgVpnRestDeliveryPointQueueBindingsResponse.md)
 - [MsgVpnRestDeliveryPointResponse](docs/MsgVpnRestDeliveryPointResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumer](docs/MsgVpnRestDeliveryPointRestConsumer.md)
 - [MsgVpnRestDeliveryPointRestConsumerLinks](docs/MsgVpnRestDeliveryPointRestConsumerLinks.md)
 - [MsgVpnRestDeliveryPointRestConsumerResponse](docs/MsgVpnRestDeliveryPointRestConsumerResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonName](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonName.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameLinks](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameLinks.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumersResponse](docs/MsgVpnRestDeliveryPointRestConsumersResponse.md)
 - [MsgVpnRestDeliveryPointsResponse](docs/MsgVpnRestDeliveryPointsResponse.md)
 - [MsgVpnSequencedTopic](docs/MsgVpnSequencedTopic.md)
 - [MsgVpnSequencedTopicLinks](docs/MsgVpnSequencedTopicLinks.md)
 - [MsgVpnSequencedTopicResponse](docs/MsgVpnSequencedTopicResponse.md)
 - [MsgVpnSequencedTopicsResponse](docs/MsgVpnSequencedTopicsResponse.md)
 - [MsgVpnTopicEndpoint](docs/MsgVpnTopicEndpoint.md)
 - [MsgVpnTopicEndpointLinks](docs/MsgVpnTopicEndpointLinks.md)
 - [MsgVpnTopicEndpointResponse](docs/MsgVpnTopicEndpointResponse.md)
 - [MsgVpnTopicEndpointsResponse](docs/MsgVpnTopicEndpointsResponse.md)
 - [MsgVpnsResponse](docs/MsgVpnsResponse.md)
 - [SempError](docs/SempError.md)
 - [SempMeta](docs/SempMeta.md)
 - [SempMetaOnlyResponse](docs/SempMetaOnlyResponse.md)
 - [SempPaging](docs/SempPaging.md)
 - [SempRequest](docs/SempRequest.md)
 - [SystemInformation](docs/SystemInformation.md)
 - [SystemInformationLinks](docs/SystemInformationLinks.md)
 - [SystemInformationResponse](docs/SystemInformationResponse.md)


## Documentation For Authorization

## basicAuth
- **Type**: HTTP basic authentication

Example
```
	auth := context.WithValue(context.TODO(), sw.ContextBasicAuth, sw.BasicAuth{
		UserName: "username",
		Password: "password",
	})
    r, err := client.Service.Operation(auth, args)
```

## Author

support@solace.com

