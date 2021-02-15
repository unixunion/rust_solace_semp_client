# Rust API client for solace_semp_client

SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any combination of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written.|See note 3 Write-Only|Attribute can only be written, not read, unless the attribute is also opaque|See the documentation for the opaque property Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version| Opaque|Attribute can be set or retrieved in opaque form when the `opaquePassword` query parameter is present|See the `opaquePassword` query parameter documentation    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    In the monitoring API, any non-identifying attribute may not be returned in a GET.  ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object (see note 5)|New attribute values|Object attributes and metadata|Set to default, with certain exceptions (see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ### opaquePassword  Attributes with the opaque property are also write-only and so cannot normally be retrieved in a GET. However, when a password is provided in the `opaquePassword` query parameter, attributes with the opaque property are retrieved in a GET in opaque form, encrypted with this password. The query parameter can also be used on a POST, PATCH, or PUT to set opaque attributes using opaque attribute values retrieved in a GET, so long as:  1. the same password that was used to retrieve the opaque attribute values is provided; and  2. the broker to which the request is being sent has the same major and minor SEMP version as the broker that produced the opaque attribute values.  The password provided in the query parameter must be a minimum of 8 characters and a maximum of 128 characters.  The query parameter can only be used in the configuration API, and only over HTTPS.  ## Help  Visit [our website](https://solace.com) to learn more about Solace.  You can also download the SEMP API specifications by clicking [here](https://solace.com/downloads/).  If you need additional support, please contact us at [support@solace.com](mailto:support@solace.com).  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|On a PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT, except in the following two cases: there is a mutual requires relationship with another non-write-only attribute and both attributes are absent from the request; or the attribute is also opaque and the `opaquePassword` query parameter is provided in the request. 5|On a PUT, if the object does not exist, it is created first.  

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 2.19
- Package version: 9.8.0-12
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
*AboutApi* | [**get_about**](docs/AboutApi.md#get_about) | **Get** /about | Get an About object.
*AboutApi* | [**get_about_api**](docs/AboutApi.md#get_about_api) | **Get** /about/api | Get an API Description object.
*AboutApi* | [**get_about_user**](docs/AboutApi.md#get_about_user) | **Get** /about/user | Get a User object.
*AboutApi* | [**get_about_user_msg_vpn**](docs/AboutApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
*AboutApi* | [**get_about_user_msg_vpns**](docs/AboutApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
*AclProfileApi* | [**create_msg_vpn_acl_profile**](docs/AclProfileApi.md#create_msg_vpn_acl_profile) | **Post** /msgVpns/{msgVpnName}/aclProfiles | Create an ACL Profile object.
*AclProfileApi* | [**create_msg_vpn_acl_profile_client_connect_exception**](docs/AclProfileApi.md#create_msg_vpn_acl_profile_client_connect_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Create a Client Connect Exception object.
*AclProfileApi* | [**create_msg_vpn_acl_profile_publish_exception**](docs/AclProfileApi.md#create_msg_vpn_acl_profile_publish_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Create a Publish Topic Exception object.
*AclProfileApi* | [**create_msg_vpn_acl_profile_publish_topic_exception**](docs/AclProfileApi.md#create_msg_vpn_acl_profile_publish_topic_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Create a Publish Topic Exception object.
*AclProfileApi* | [**create_msg_vpn_acl_profile_subscribe_exception**](docs/AclProfileApi.md#create_msg_vpn_acl_profile_subscribe_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Create a Subscribe Topic Exception object.
*AclProfileApi* | [**create_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/AclProfileApi.md#create_msg_vpn_acl_profile_subscribe_share_name_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Create a Subscribe Share Name Exception object.
*AclProfileApi* | [**create_msg_vpn_acl_profile_subscribe_topic_exception**](docs/AclProfileApi.md#create_msg_vpn_acl_profile_subscribe_topic_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Create a Subscribe Topic Exception object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Delete an ACL Profile object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile_client_connect_exception**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile_client_connect_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Delete a Client Connect Exception object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile_publish_exception**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile_publish_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Delete a Publish Topic Exception object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile_publish_topic_exception**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile_publish_topic_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Delete a Publish Topic Exception object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile_subscribe_exception**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile_subscribe_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Delete a Subscribe Topic Exception object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile_subscribe_share_name_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Delete a Subscribe Share Name Exception object.
*AclProfileApi* | [**delete_msg_vpn_acl_profile_subscribe_topic_exception**](docs/AclProfileApi.md#delete_msg_vpn_acl_profile_subscribe_topic_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Delete a Subscribe Topic Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile**](docs/AclProfileApi.md#get_msg_vpn_acl_profile) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Get an ACL Profile object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_client_connect_exception**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_client_connect_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Get a Client Connect Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_client_connect_exceptions**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_client_connect_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Get a list of Client Connect Exception objects.
*AclProfileApi* | [**get_msg_vpn_acl_profile_publish_exception**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_publish_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Get a Publish Topic Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_publish_exceptions**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_publish_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Get a list of Publish Topic Exception objects.
*AclProfileApi* | [**get_msg_vpn_acl_profile_publish_topic_exception**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_publish_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Get a Publish Topic Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_publish_topic_exceptions**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_publish_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Get a list of Publish Topic Exception objects.
*AclProfileApi* | [**get_msg_vpn_acl_profile_subscribe_exception**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Get a Subscribe Topic Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_subscribe_exceptions**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Get a list of Subscribe Topic Exception objects.
*AclProfileApi* | [**get_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Get a Subscribe Share Name Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_subscribe_share_name_exceptions**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Get a list of Subscribe Share Name Exception objects.
*AclProfileApi* | [**get_msg_vpn_acl_profile_subscribe_topic_exception**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_subscribe_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Get a Subscribe Topic Exception object.
*AclProfileApi* | [**get_msg_vpn_acl_profile_subscribe_topic_exceptions**](docs/AclProfileApi.md#get_msg_vpn_acl_profile_subscribe_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Get a list of Subscribe Topic Exception objects.
*AclProfileApi* | [**get_msg_vpn_acl_profiles**](docs/AclProfileApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Get a list of ACL Profile objects.
*AclProfileApi* | [**replace_msg_vpn_acl_profile**](docs/AclProfileApi.md#replace_msg_vpn_acl_profile) | **Put** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Replace an ACL Profile object.
*AclProfileApi* | [**update_msg_vpn_acl_profile**](docs/AclProfileApi.md#update_msg_vpn_acl_profile) | **Patch** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Update an ACL Profile object.
*AllApi* | [**create_cert_authority**](docs/AllApi.md#create_cert_authority) | **Post** /certAuthorities | Create a Certificate Authority object.
*AllApi* | [**create_cert_authority_ocsp_tls_trusted_common_name**](docs/AllApi.md#create_cert_authority_ocsp_tls_trusted_common_name) | **Post** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Create an OCSP Responder Trusted Common Name object.
*AllApi* | [**create_client_cert_authority**](docs/AllApi.md#create_client_cert_authority) | **Post** /clientCertAuthorities | Create a Client Certificate Authority object.
*AllApi* | [**create_client_cert_authority_ocsp_tls_trusted_common_name**](docs/AllApi.md#create_client_cert_authority_ocsp_tls_trusted_common_name) | **Post** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Create an OCSP Responder Trusted Common Name object.
*AllApi* | [**create_dmr_cluster**](docs/AllApi.md#create_dmr_cluster) | **Post** /dmrClusters | Create a Cluster object.
*AllApi* | [**create_dmr_cluster_link**](docs/AllApi.md#create_dmr_cluster_link) | **Post** /dmrClusters/{dmrClusterName}/links | Create a Link object.
*AllApi* | [**create_dmr_cluster_link_remote_address**](docs/AllApi.md#create_dmr_cluster_link_remote_address) | **Post** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Create a Remote Address object.
*AllApi* | [**create_dmr_cluster_link_tls_trusted_common_name**](docs/AllApi.md#create_dmr_cluster_link_tls_trusted_common_name) | **Post** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*AllApi* | [**create_domain_cert_authority**](docs/AllApi.md#create_domain_cert_authority) | **Post** /domainCertAuthorities | Create a Domain Certificate Authority object.
*AllApi* | [**create_msg_vpn**](docs/AllApi.md#create_msg_vpn) | **Post** /msgVpns | Create a Message VPN object.
*AllApi* | [**create_msg_vpn_acl_profile**](docs/AllApi.md#create_msg_vpn_acl_profile) | **Post** /msgVpns/{msgVpnName}/aclProfiles | Create an ACL Profile object.
*AllApi* | [**create_msg_vpn_acl_profile_client_connect_exception**](docs/AllApi.md#create_msg_vpn_acl_profile_client_connect_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Create a Client Connect Exception object.
*AllApi* | [**create_msg_vpn_acl_profile_publish_exception**](docs/AllApi.md#create_msg_vpn_acl_profile_publish_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Create a Publish Topic Exception object.
*AllApi* | [**create_msg_vpn_acl_profile_publish_topic_exception**](docs/AllApi.md#create_msg_vpn_acl_profile_publish_topic_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Create a Publish Topic Exception object.
*AllApi* | [**create_msg_vpn_acl_profile_subscribe_exception**](docs/AllApi.md#create_msg_vpn_acl_profile_subscribe_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Create a Subscribe Topic Exception object.
*AllApi* | [**create_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/AllApi.md#create_msg_vpn_acl_profile_subscribe_share_name_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Create a Subscribe Share Name Exception object.
*AllApi* | [**create_msg_vpn_acl_profile_subscribe_topic_exception**](docs/AllApi.md#create_msg_vpn_acl_profile_subscribe_topic_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Create a Subscribe Topic Exception object.
*AllApi* | [**create_msg_vpn_authentication_oauth_provider**](docs/AllApi.md#create_msg_vpn_authentication_oauth_provider) | **Post** /msgVpns/{msgVpnName}/authenticationOauthProviders | Create an OAuth Provider object.
*AllApi* | [**create_msg_vpn_authorization_group**](docs/AllApi.md#create_msg_vpn_authorization_group) | **Post** /msgVpns/{msgVpnName}/authorizationGroups | Create an LDAP Authorization Group object.
*AllApi* | [**create_msg_vpn_bridge**](docs/AllApi.md#create_msg_vpn_bridge) | **Post** /msgVpns/{msgVpnName}/bridges | Create a Bridge object.
*AllApi* | [**create_msg_vpn_bridge_remote_msg_vpn**](docs/AllApi.md#create_msg_vpn_bridge_remote_msg_vpn) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Create a Remote Message VPN object.
*AllApi* | [**create_msg_vpn_bridge_remote_subscription**](docs/AllApi.md#create_msg_vpn_bridge_remote_subscription) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Create a Remote Subscription object.
*AllApi* | [**create_msg_vpn_bridge_tls_trusted_common_name**](docs/AllApi.md#create_msg_vpn_bridge_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*AllApi* | [**create_msg_vpn_client_profile**](docs/AllApi.md#create_msg_vpn_client_profile) | **Post** /msgVpns/{msgVpnName}/clientProfiles | Create a Client Profile object.
*AllApi* | [**create_msg_vpn_client_username**](docs/AllApi.md#create_msg_vpn_client_username) | **Post** /msgVpns/{msgVpnName}/clientUsernames | Create a Client Username object.
*AllApi* | [**create_msg_vpn_distributed_cache**](docs/AllApi.md#create_msg_vpn_distributed_cache) | **Post** /msgVpns/{msgVpnName}/distributedCaches | Create a Distributed Cache object.
*AllApi* | [**create_msg_vpn_distributed_cache_cluster**](docs/AllApi.md#create_msg_vpn_distributed_cache_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Create a Cache Cluster object.
*AllApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/AllApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Create a Home Cache Cluster object.
*AllApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/AllApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Create a Topic Prefix object.
*AllApi* | [**create_msg_vpn_distributed_cache_cluster_instance**](docs/AllApi.md#create_msg_vpn_distributed_cache_cluster_instance) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Create a Cache Instance object.
*AllApi* | [**create_msg_vpn_distributed_cache_cluster_topic**](docs/AllApi.md#create_msg_vpn_distributed_cache_cluster_topic) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Create a Topic object.
*AllApi* | [**create_msg_vpn_dmr_bridge**](docs/AllApi.md#create_msg_vpn_dmr_bridge) | **Post** /msgVpns/{msgVpnName}/dmrBridges | Create a DMR Bridge object.
*AllApi* | [**create_msg_vpn_jndi_connection_factory**](docs/AllApi.md#create_msg_vpn_jndi_connection_factory) | **Post** /msgVpns/{msgVpnName}/jndiConnectionFactories | Create a JNDI Connection Factory object.
*AllApi* | [**create_msg_vpn_jndi_queue**](docs/AllApi.md#create_msg_vpn_jndi_queue) | **Post** /msgVpns/{msgVpnName}/jndiQueues | Create a JNDI Queue object.
*AllApi* | [**create_msg_vpn_jndi_topic**](docs/AllApi.md#create_msg_vpn_jndi_topic) | **Post** /msgVpns/{msgVpnName}/jndiTopics | Create a JNDI Topic object.
*AllApi* | [**create_msg_vpn_mqtt_retain_cache**](docs/AllApi.md#create_msg_vpn_mqtt_retain_cache) | **Post** /msgVpns/{msgVpnName}/mqttRetainCaches | Create an MQTT Retain Cache object.
*AllApi* | [**create_msg_vpn_mqtt_session**](docs/AllApi.md#create_msg_vpn_mqtt_session) | **Post** /msgVpns/{msgVpnName}/mqttSessions | Create an MQTT Session object.
*AllApi* | [**create_msg_vpn_mqtt_session_subscription**](docs/AllApi.md#create_msg_vpn_mqtt_session_subscription) | **Post** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Create a Subscription object.
*AllApi* | [**create_msg_vpn_queue**](docs/AllApi.md#create_msg_vpn_queue) | **Post** /msgVpns/{msgVpnName}/queues | Create a Queue object.
*AllApi* | [**create_msg_vpn_queue_subscription**](docs/AllApi.md#create_msg_vpn_queue_subscription) | **Post** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Create a Queue Subscription object.
*AllApi* | [**create_msg_vpn_queue_template**](docs/AllApi.md#create_msg_vpn_queue_template) | **Post** /msgVpns/{msgVpnName}/queueTemplates | Create a Queue Template object.
*AllApi* | [**create_msg_vpn_replay_log**](docs/AllApi.md#create_msg_vpn_replay_log) | **Post** /msgVpns/{msgVpnName}/replayLogs | Create a Replay Log object.
*AllApi* | [**create_msg_vpn_replicated_topic**](docs/AllApi.md#create_msg_vpn_replicated_topic) | **Post** /msgVpns/{msgVpnName}/replicatedTopics | Create a Replicated Topic object.
*AllApi* | [**create_msg_vpn_rest_delivery_point**](docs/AllApi.md#create_msg_vpn_rest_delivery_point) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints | Create a REST Delivery Point object.
*AllApi* | [**create_msg_vpn_rest_delivery_point_queue_binding**](docs/AllApi.md#create_msg_vpn_rest_delivery_point_queue_binding) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Create a Queue Binding object.
*AllApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer**](docs/AllApi.md#create_msg_vpn_rest_delivery_point_rest_consumer) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Create a REST Consumer object.
*AllApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/AllApi.md#create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*AllApi* | [**create_msg_vpn_sequenced_topic**](docs/AllApi.md#create_msg_vpn_sequenced_topic) | **Post** /msgVpns/{msgVpnName}/sequencedTopics | Create a Sequenced Topic object.
*AllApi* | [**create_msg_vpn_topic_endpoint**](docs/AllApi.md#create_msg_vpn_topic_endpoint) | **Post** /msgVpns/{msgVpnName}/topicEndpoints | Create a Topic Endpoint object.
*AllApi* | [**create_msg_vpn_topic_endpoint_template**](docs/AllApi.md#create_msg_vpn_topic_endpoint_template) | **Post** /msgVpns/{msgVpnName}/topicEndpointTemplates | Create a Topic Endpoint Template object.
*AllApi* | [**create_virtual_hostname**](docs/AllApi.md#create_virtual_hostname) | **Post** /virtualHostnames | Create a Virtual Hostname object.
*AllApi* | [**delete_cert_authority**](docs/AllApi.md#delete_cert_authority) | **Delete** /certAuthorities/{certAuthorityName} | Delete a Certificate Authority object.
*AllApi* | [**delete_cert_authority_ocsp_tls_trusted_common_name**](docs/AllApi.md#delete_cert_authority_ocsp_tls_trusted_common_name) | **Delete** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Delete an OCSP Responder Trusted Common Name object.
*AllApi* | [**delete_client_cert_authority**](docs/AllApi.md#delete_client_cert_authority) | **Delete** /clientCertAuthorities/{certAuthorityName} | Delete a Client Certificate Authority object.
*AllApi* | [**delete_client_cert_authority_ocsp_tls_trusted_common_name**](docs/AllApi.md#delete_client_cert_authority_ocsp_tls_trusted_common_name) | **Delete** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Delete an OCSP Responder Trusted Common Name object.
*AllApi* | [**delete_dmr_cluster**](docs/AllApi.md#delete_dmr_cluster) | **Delete** /dmrClusters/{dmrClusterName} | Delete a Cluster object.
*AllApi* | [**delete_dmr_cluster_link**](docs/AllApi.md#delete_dmr_cluster_link) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Delete a Link object.
*AllApi* | [**delete_dmr_cluster_link_remote_address**](docs/AllApi.md#delete_dmr_cluster_link_remote_address) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Delete a Remote Address object.
*AllApi* | [**delete_dmr_cluster_link_tls_trusted_common_name**](docs/AllApi.md#delete_dmr_cluster_link_tls_trusted_common_name) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*AllApi* | [**delete_domain_cert_authority**](docs/AllApi.md#delete_domain_cert_authority) | **Delete** /domainCertAuthorities/{certAuthorityName} | Delete a Domain Certificate Authority object.
*AllApi* | [**delete_msg_vpn**](docs/AllApi.md#delete_msg_vpn) | **Delete** /msgVpns/{msgVpnName} | Delete a Message VPN object.
*AllApi* | [**delete_msg_vpn_acl_profile**](docs/AllApi.md#delete_msg_vpn_acl_profile) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Delete an ACL Profile object.
*AllApi* | [**delete_msg_vpn_acl_profile_client_connect_exception**](docs/AllApi.md#delete_msg_vpn_acl_profile_client_connect_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Delete a Client Connect Exception object.
*AllApi* | [**delete_msg_vpn_acl_profile_publish_exception**](docs/AllApi.md#delete_msg_vpn_acl_profile_publish_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Delete a Publish Topic Exception object.
*AllApi* | [**delete_msg_vpn_acl_profile_publish_topic_exception**](docs/AllApi.md#delete_msg_vpn_acl_profile_publish_topic_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Delete a Publish Topic Exception object.
*AllApi* | [**delete_msg_vpn_acl_profile_subscribe_exception**](docs/AllApi.md#delete_msg_vpn_acl_profile_subscribe_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Delete a Subscribe Topic Exception object.
*AllApi* | [**delete_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/AllApi.md#delete_msg_vpn_acl_profile_subscribe_share_name_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Delete a Subscribe Share Name Exception object.
*AllApi* | [**delete_msg_vpn_acl_profile_subscribe_topic_exception**](docs/AllApi.md#delete_msg_vpn_acl_profile_subscribe_topic_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Delete a Subscribe Topic Exception object.
*AllApi* | [**delete_msg_vpn_authentication_oauth_provider**](docs/AllApi.md#delete_msg_vpn_authentication_oauth_provider) | **Delete** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Delete an OAuth Provider object.
*AllApi* | [**delete_msg_vpn_authorization_group**](docs/AllApi.md#delete_msg_vpn_authorization_group) | **Delete** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Delete an LDAP Authorization Group object.
*AllApi* | [**delete_msg_vpn_bridge**](docs/AllApi.md#delete_msg_vpn_bridge) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Delete a Bridge object.
*AllApi* | [**delete_msg_vpn_bridge_remote_msg_vpn**](docs/AllApi.md#delete_msg_vpn_bridge_remote_msg_vpn) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Delete a Remote Message VPN object.
*AllApi* | [**delete_msg_vpn_bridge_remote_subscription**](docs/AllApi.md#delete_msg_vpn_bridge_remote_subscription) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Delete a Remote Subscription object.
*AllApi* | [**delete_msg_vpn_bridge_tls_trusted_common_name**](docs/AllApi.md#delete_msg_vpn_bridge_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*AllApi* | [**delete_msg_vpn_client_profile**](docs/AllApi.md#delete_msg_vpn_client_profile) | **Delete** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Delete a Client Profile object.
*AllApi* | [**delete_msg_vpn_client_username**](docs/AllApi.md#delete_msg_vpn_client_username) | **Delete** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Delete a Client Username object.
*AllApi* | [**delete_msg_vpn_distributed_cache**](docs/AllApi.md#delete_msg_vpn_distributed_cache) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Delete a Distributed Cache object.
*AllApi* | [**delete_msg_vpn_distributed_cache_cluster**](docs/AllApi.md#delete_msg_vpn_distributed_cache_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Delete a Cache Cluster object.
*AllApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/AllApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Delete a Home Cache Cluster object.
*AllApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/AllApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Delete a Topic Prefix object.
*AllApi* | [**delete_msg_vpn_distributed_cache_cluster_instance**](docs/AllApi.md#delete_msg_vpn_distributed_cache_cluster_instance) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Delete a Cache Instance object.
*AllApi* | [**delete_msg_vpn_distributed_cache_cluster_topic**](docs/AllApi.md#delete_msg_vpn_distributed_cache_cluster_topic) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Delete a Topic object.
*AllApi* | [**delete_msg_vpn_dmr_bridge**](docs/AllApi.md#delete_msg_vpn_dmr_bridge) | **Delete** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Delete a DMR Bridge object.
*AllApi* | [**delete_msg_vpn_jndi_connection_factory**](docs/AllApi.md#delete_msg_vpn_jndi_connection_factory) | **Delete** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Delete a JNDI Connection Factory object.
*AllApi* | [**delete_msg_vpn_jndi_queue**](docs/AllApi.md#delete_msg_vpn_jndi_queue) | **Delete** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Delete a JNDI Queue object.
*AllApi* | [**delete_msg_vpn_jndi_topic**](docs/AllApi.md#delete_msg_vpn_jndi_topic) | **Delete** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Delete a JNDI Topic object.
*AllApi* | [**delete_msg_vpn_mqtt_retain_cache**](docs/AllApi.md#delete_msg_vpn_mqtt_retain_cache) | **Delete** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Delete an MQTT Retain Cache object.
*AllApi* | [**delete_msg_vpn_mqtt_session**](docs/AllApi.md#delete_msg_vpn_mqtt_session) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Delete an MQTT Session object.
*AllApi* | [**delete_msg_vpn_mqtt_session_subscription**](docs/AllApi.md#delete_msg_vpn_mqtt_session_subscription) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Delete a Subscription object.
*AllApi* | [**delete_msg_vpn_queue**](docs/AllApi.md#delete_msg_vpn_queue) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName} | Delete a Queue object.
*AllApi* | [**delete_msg_vpn_queue_subscription**](docs/AllApi.md#delete_msg_vpn_queue_subscription) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Delete a Queue Subscription object.
*AllApi* | [**delete_msg_vpn_queue_template**](docs/AllApi.md#delete_msg_vpn_queue_template) | **Delete** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Delete a Queue Template object.
*AllApi* | [**delete_msg_vpn_replay_log**](docs/AllApi.md#delete_msg_vpn_replay_log) | **Delete** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Delete a Replay Log object.
*AllApi* | [**delete_msg_vpn_replicated_topic**](docs/AllApi.md#delete_msg_vpn_replicated_topic) | **Delete** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Delete a Replicated Topic object.
*AllApi* | [**delete_msg_vpn_rest_delivery_point**](docs/AllApi.md#delete_msg_vpn_rest_delivery_point) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Delete a REST Delivery Point object.
*AllApi* | [**delete_msg_vpn_rest_delivery_point_queue_binding**](docs/AllApi.md#delete_msg_vpn_rest_delivery_point_queue_binding) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Delete a Queue Binding object.
*AllApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer**](docs/AllApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Delete a REST Consumer object.
*AllApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/AllApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*AllApi* | [**delete_msg_vpn_sequenced_topic**](docs/AllApi.md#delete_msg_vpn_sequenced_topic) | **Delete** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Delete a Sequenced Topic object.
*AllApi* | [**delete_msg_vpn_topic_endpoint**](docs/AllApi.md#delete_msg_vpn_topic_endpoint) | **Delete** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Delete a Topic Endpoint object.
*AllApi* | [**delete_msg_vpn_topic_endpoint_template**](docs/AllApi.md#delete_msg_vpn_topic_endpoint_template) | **Delete** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Delete a Topic Endpoint Template object.
*AllApi* | [**delete_virtual_hostname**](docs/AllApi.md#delete_virtual_hostname) | **Delete** /virtualHostnames/{virtualHostname} | Delete a Virtual Hostname object.
*AllApi* | [**get_about**](docs/AllApi.md#get_about) | **Get** /about | Get an About object.
*AllApi* | [**get_about_api**](docs/AllApi.md#get_about_api) | **Get** /about/api | Get an API Description object.
*AllApi* | [**get_about_user**](docs/AllApi.md#get_about_user) | **Get** /about/user | Get a User object.
*AllApi* | [**get_about_user_msg_vpn**](docs/AllApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Get a User Message VPN object.
*AllApi* | [**get_about_user_msg_vpns**](docs/AllApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Get a list of User Message VPN objects.
*AllApi* | [**get_broker**](docs/AllApi.md#get_broker) | **Get** / | Get a Broker object.
*AllApi* | [**get_cert_authorities**](docs/AllApi.md#get_cert_authorities) | **Get** /certAuthorities | Get a list of Certificate Authority objects.
*AllApi* | [**get_cert_authority**](docs/AllApi.md#get_cert_authority) | **Get** /certAuthorities/{certAuthorityName} | Get a Certificate Authority object.
*AllApi* | [**get_cert_authority_ocsp_tls_trusted_common_name**](docs/AllApi.md#get_cert_authority_ocsp_tls_trusted_common_name) | **Get** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Get an OCSP Responder Trusted Common Name object.
*AllApi* | [**get_cert_authority_ocsp_tls_trusted_common_names**](docs/AllApi.md#get_cert_authority_ocsp_tls_trusted_common_names) | **Get** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Get a list of OCSP Responder Trusted Common Name objects.
*AllApi* | [**get_client_cert_authorities**](docs/AllApi.md#get_client_cert_authorities) | **Get** /clientCertAuthorities | Get a list of Client Certificate Authority objects.
*AllApi* | [**get_client_cert_authority**](docs/AllApi.md#get_client_cert_authority) | **Get** /clientCertAuthorities/{certAuthorityName} | Get a Client Certificate Authority object.
*AllApi* | [**get_client_cert_authority_ocsp_tls_trusted_common_name**](docs/AllApi.md#get_client_cert_authority_ocsp_tls_trusted_common_name) | **Get** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Get an OCSP Responder Trusted Common Name object.
*AllApi* | [**get_client_cert_authority_ocsp_tls_trusted_common_names**](docs/AllApi.md#get_client_cert_authority_ocsp_tls_trusted_common_names) | **Get** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Get a list of OCSP Responder Trusted Common Name objects.
*AllApi* | [**get_dmr_cluster**](docs/AllApi.md#get_dmr_cluster) | **Get** /dmrClusters/{dmrClusterName} | Get a Cluster object.
*AllApi* | [**get_dmr_cluster_link**](docs/AllApi.md#get_dmr_cluster_link) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Get a Link object.
*AllApi* | [**get_dmr_cluster_link_remote_address**](docs/AllApi.md#get_dmr_cluster_link_remote_address) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Get a Remote Address object.
*AllApi* | [**get_dmr_cluster_link_remote_addresses**](docs/AllApi.md#get_dmr_cluster_link_remote_addresses) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Get a list of Remote Address objects.
*AllApi* | [**get_dmr_cluster_link_tls_trusted_common_name**](docs/AllApi.md#get_dmr_cluster_link_tls_trusted_common_name) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*AllApi* | [**get_dmr_cluster_link_tls_trusted_common_names**](docs/AllApi.md#get_dmr_cluster_link_tls_trusted_common_names) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*AllApi* | [**get_dmr_cluster_links**](docs/AllApi.md#get_dmr_cluster_links) | **Get** /dmrClusters/{dmrClusterName}/links | Get a list of Link objects.
*AllApi* | [**get_dmr_clusters**](docs/AllApi.md#get_dmr_clusters) | **Get** /dmrClusters | Get a list of Cluster objects.
*AllApi* | [**get_domain_cert_authorities**](docs/AllApi.md#get_domain_cert_authorities) | **Get** /domainCertAuthorities | Get a list of Domain Certificate Authority objects.
*AllApi* | [**get_domain_cert_authority**](docs/AllApi.md#get_domain_cert_authority) | **Get** /domainCertAuthorities/{certAuthorityName} | Get a Domain Certificate Authority object.
*AllApi* | [**get_msg_vpn**](docs/AllApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Get a Message VPN object.
*AllApi* | [**get_msg_vpn_acl_profile**](docs/AllApi.md#get_msg_vpn_acl_profile) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Get an ACL Profile object.
*AllApi* | [**get_msg_vpn_acl_profile_client_connect_exception**](docs/AllApi.md#get_msg_vpn_acl_profile_client_connect_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Get a Client Connect Exception object.
*AllApi* | [**get_msg_vpn_acl_profile_client_connect_exceptions**](docs/AllApi.md#get_msg_vpn_acl_profile_client_connect_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Get a list of Client Connect Exception objects.
*AllApi* | [**get_msg_vpn_acl_profile_publish_exception**](docs/AllApi.md#get_msg_vpn_acl_profile_publish_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Get a Publish Topic Exception object.
*AllApi* | [**get_msg_vpn_acl_profile_publish_exceptions**](docs/AllApi.md#get_msg_vpn_acl_profile_publish_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Get a list of Publish Topic Exception objects.
*AllApi* | [**get_msg_vpn_acl_profile_publish_topic_exception**](docs/AllApi.md#get_msg_vpn_acl_profile_publish_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Get a Publish Topic Exception object.
*AllApi* | [**get_msg_vpn_acl_profile_publish_topic_exceptions**](docs/AllApi.md#get_msg_vpn_acl_profile_publish_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Get a list of Publish Topic Exception objects.
*AllApi* | [**get_msg_vpn_acl_profile_subscribe_exception**](docs/AllApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Get a Subscribe Topic Exception object.
*AllApi* | [**get_msg_vpn_acl_profile_subscribe_exceptions**](docs/AllApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Get a list of Subscribe Topic Exception objects.
*AllApi* | [**get_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/AllApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Get a Subscribe Share Name Exception object.
*AllApi* | [**get_msg_vpn_acl_profile_subscribe_share_name_exceptions**](docs/AllApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Get a list of Subscribe Share Name Exception objects.
*AllApi* | [**get_msg_vpn_acl_profile_subscribe_topic_exception**](docs/AllApi.md#get_msg_vpn_acl_profile_subscribe_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Get a Subscribe Topic Exception object.
*AllApi* | [**get_msg_vpn_acl_profile_subscribe_topic_exceptions**](docs/AllApi.md#get_msg_vpn_acl_profile_subscribe_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Get a list of Subscribe Topic Exception objects.
*AllApi* | [**get_msg_vpn_acl_profiles**](docs/AllApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Get a list of ACL Profile objects.
*AllApi* | [**get_msg_vpn_authentication_oauth_provider**](docs/AllApi.md#get_msg_vpn_authentication_oauth_provider) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Get an OAuth Provider object.
*AllApi* | [**get_msg_vpn_authentication_oauth_providers**](docs/AllApi.md#get_msg_vpn_authentication_oauth_providers) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders | Get a list of OAuth Provider objects.
*AllApi* | [**get_msg_vpn_authorization_group**](docs/AllApi.md#get_msg_vpn_authorization_group) | **Get** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Get an LDAP Authorization Group object.
*AllApi* | [**get_msg_vpn_authorization_groups**](docs/AllApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Get a list of LDAP Authorization Group objects.
*AllApi* | [**get_msg_vpn_bridge**](docs/AllApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
*AllApi* | [**get_msg_vpn_bridge_remote_msg_vpn**](docs/AllApi.md#get_msg_vpn_bridge_remote_msg_vpn) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Get a Remote Message VPN object.
*AllApi* | [**get_msg_vpn_bridge_remote_msg_vpns**](docs/AllApi.md#get_msg_vpn_bridge_remote_msg_vpns) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Get a list of Remote Message VPN objects.
*AllApi* | [**get_msg_vpn_bridge_remote_subscription**](docs/AllApi.md#get_msg_vpn_bridge_remote_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Get a Remote Subscription object.
*AllApi* | [**get_msg_vpn_bridge_remote_subscriptions**](docs/AllApi.md#get_msg_vpn_bridge_remote_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Get a list of Remote Subscription objects.
*AllApi* | [**get_msg_vpn_bridge_tls_trusted_common_name**](docs/AllApi.md#get_msg_vpn_bridge_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*AllApi* | [**get_msg_vpn_bridge_tls_trusted_common_names**](docs/AllApi.md#get_msg_vpn_bridge_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*AllApi* | [**get_msg_vpn_bridges**](docs/AllApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*AllApi* | [**get_msg_vpn_client_profile**](docs/AllApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Get a Client Profile object.
*AllApi* | [**get_msg_vpn_client_profiles**](docs/AllApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Get a list of Client Profile objects.
*AllApi* | [**get_msg_vpn_client_username**](docs/AllApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Get a Client Username object.
*AllApi* | [**get_msg_vpn_client_usernames**](docs/AllApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Get a list of Client Username objects.
*AllApi* | [**get_msg_vpn_distributed_cache**](docs/AllApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Get a Home Cache Cluster object.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Get a Topic Prefix object.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Get a list of Topic Prefix objects.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Get a list of Home Cache Cluster objects.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_instance**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_instances**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_topic**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_topic) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Get a Topic object.
*AllApi* | [**get_msg_vpn_distributed_cache_cluster_topics**](docs/AllApi.md#get_msg_vpn_distributed_cache_cluster_topics) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Get a list of Topic objects.
*AllApi* | [**get_msg_vpn_distributed_cache_clusters**](docs/AllApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
*AllApi* | [**get_msg_vpn_distributed_caches**](docs/AllApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*AllApi* | [**get_msg_vpn_dmr_bridge**](docs/AllApi.md#get_msg_vpn_dmr_bridge) | **Get** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Get a DMR Bridge object.
*AllApi* | [**get_msg_vpn_dmr_bridges**](docs/AllApi.md#get_msg_vpn_dmr_bridges) | **Get** /msgVpns/{msgVpnName}/dmrBridges | Get a list of DMR Bridge objects.
*AllApi* | [**get_msg_vpn_jndi_connection_factories**](docs/AllApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Get a list of JNDI Connection Factory objects.
*AllApi* | [**get_msg_vpn_jndi_connection_factory**](docs/AllApi.md#get_msg_vpn_jndi_connection_factory) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Get a JNDI Connection Factory object.
*AllApi* | [**get_msg_vpn_jndi_queue**](docs/AllApi.md#get_msg_vpn_jndi_queue) | **Get** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Get a JNDI Queue object.
*AllApi* | [**get_msg_vpn_jndi_queues**](docs/AllApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Get a list of JNDI Queue objects.
*AllApi* | [**get_msg_vpn_jndi_topic**](docs/AllApi.md#get_msg_vpn_jndi_topic) | **Get** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Get a JNDI Topic object.
*AllApi* | [**get_msg_vpn_jndi_topics**](docs/AllApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Get a list of JNDI Topic objects.
*AllApi* | [**get_msg_vpn_mqtt_retain_cache**](docs/AllApi.md#get_msg_vpn_mqtt_retain_cache) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Get an MQTT Retain Cache object.
*AllApi* | [**get_msg_vpn_mqtt_retain_caches**](docs/AllApi.md#get_msg_vpn_mqtt_retain_caches) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches | Get a list of MQTT Retain Cache objects.
*AllApi* | [**get_msg_vpn_mqtt_session**](docs/AllApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
*AllApi* | [**get_msg_vpn_mqtt_session_subscription**](docs/AllApi.md#get_msg_vpn_mqtt_session_subscription) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Get a Subscription object.
*AllApi* | [**get_msg_vpn_mqtt_session_subscriptions**](docs/AllApi.md#get_msg_vpn_mqtt_session_subscriptions) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Get a list of Subscription objects.
*AllApi* | [**get_msg_vpn_mqtt_sessions**](docs/AllApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*AllApi* | [**get_msg_vpn_queue**](docs/AllApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
*AllApi* | [**get_msg_vpn_queue_subscription**](docs/AllApi.md#get_msg_vpn_queue_subscription) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Get a Queue Subscription object.
*AllApi* | [**get_msg_vpn_queue_subscriptions**](docs/AllApi.md#get_msg_vpn_queue_subscriptions) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Get a list of Queue Subscription objects.
*AllApi* | [**get_msg_vpn_queue_template**](docs/AllApi.md#get_msg_vpn_queue_template) | **Get** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Get a Queue Template object.
*AllApi* | [**get_msg_vpn_queue_templates**](docs/AllApi.md#get_msg_vpn_queue_templates) | **Get** /msgVpns/{msgVpnName}/queueTemplates | Get a list of Queue Template objects.
*AllApi* | [**get_msg_vpn_queues**](docs/AllApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*AllApi* | [**get_msg_vpn_replay_log**](docs/AllApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
*AllApi* | [**get_msg_vpn_replay_logs**](docs/AllApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*AllApi* | [**get_msg_vpn_replicated_topic**](docs/AllApi.md#get_msg_vpn_replicated_topic) | **Get** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Get a Replicated Topic object.
*AllApi* | [**get_msg_vpn_replicated_topics**](docs/AllApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Get a list of Replicated Topic objects.
*AllApi* | [**get_msg_vpn_rest_delivery_point**](docs/AllApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
*AllApi* | [**get_msg_vpn_rest_delivery_point_queue_binding**](docs/AllApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Get a Queue Binding object.
*AllApi* | [**get_msg_vpn_rest_delivery_point_queue_bindings**](docs/AllApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Get a list of Queue Binding objects.
*AllApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer**](docs/AllApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
*AllApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/AllApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*AllApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](docs/AllApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*AllApi* | [**get_msg_vpn_rest_delivery_point_rest_consumers**](docs/AllApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
*AllApi* | [**get_msg_vpn_rest_delivery_points**](docs/AllApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*AllApi* | [**get_msg_vpn_sequenced_topic**](docs/AllApi.md#get_msg_vpn_sequenced_topic) | **Get** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Get a Sequenced Topic object.
*AllApi* | [**get_msg_vpn_sequenced_topics**](docs/AllApi.md#get_msg_vpn_sequenced_topics) | **Get** /msgVpns/{msgVpnName}/sequencedTopics | Get a list of Sequenced Topic objects.
*AllApi* | [**get_msg_vpn_topic_endpoint**](docs/AllApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
*AllApi* | [**get_msg_vpn_topic_endpoint_template**](docs/AllApi.md#get_msg_vpn_topic_endpoint_template) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Get a Topic Endpoint Template object.
*AllApi* | [**get_msg_vpn_topic_endpoint_templates**](docs/AllApi.md#get_msg_vpn_topic_endpoint_templates) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates | Get a list of Topic Endpoint Template objects.
*AllApi* | [**get_msg_vpn_topic_endpoints**](docs/AllApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.
*AllApi* | [**get_msg_vpns**](docs/AllApi.md#get_msg_vpns) | **Get** /msgVpns | Get a list of Message VPN objects.
*AllApi* | [**get_virtual_hostname**](docs/AllApi.md#get_virtual_hostname) | **Get** /virtualHostnames/{virtualHostname} | Get a Virtual Hostname object.
*AllApi* | [**get_virtual_hostnames**](docs/AllApi.md#get_virtual_hostnames) | **Get** /virtualHostnames | Get a list of Virtual Hostname objects.
*AllApi* | [**replace_cert_authority**](docs/AllApi.md#replace_cert_authority) | **Put** /certAuthorities/{certAuthorityName} | Replace a Certificate Authority object.
*AllApi* | [**replace_client_cert_authority**](docs/AllApi.md#replace_client_cert_authority) | **Put** /clientCertAuthorities/{certAuthorityName} | Replace a Client Certificate Authority object.
*AllApi* | [**replace_dmr_cluster**](docs/AllApi.md#replace_dmr_cluster) | **Put** /dmrClusters/{dmrClusterName} | Replace a Cluster object.
*AllApi* | [**replace_dmr_cluster_link**](docs/AllApi.md#replace_dmr_cluster_link) | **Put** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Replace a Link object.
*AllApi* | [**replace_domain_cert_authority**](docs/AllApi.md#replace_domain_cert_authority) | **Put** /domainCertAuthorities/{certAuthorityName} | Replace a Domain Certificate Authority object.
*AllApi* | [**replace_msg_vpn**](docs/AllApi.md#replace_msg_vpn) | **Put** /msgVpns/{msgVpnName} | Replace a Message VPN object.
*AllApi* | [**replace_msg_vpn_acl_profile**](docs/AllApi.md#replace_msg_vpn_acl_profile) | **Put** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Replace an ACL Profile object.
*AllApi* | [**replace_msg_vpn_authentication_oauth_provider**](docs/AllApi.md#replace_msg_vpn_authentication_oauth_provider) | **Put** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Replace an OAuth Provider object.
*AllApi* | [**replace_msg_vpn_authorization_group**](docs/AllApi.md#replace_msg_vpn_authorization_group) | **Put** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Replace an LDAP Authorization Group object.
*AllApi* | [**replace_msg_vpn_bridge**](docs/AllApi.md#replace_msg_vpn_bridge) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Replace a Bridge object.
*AllApi* | [**replace_msg_vpn_bridge_remote_msg_vpn**](docs/AllApi.md#replace_msg_vpn_bridge_remote_msg_vpn) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Replace a Remote Message VPN object.
*AllApi* | [**replace_msg_vpn_client_profile**](docs/AllApi.md#replace_msg_vpn_client_profile) | **Put** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Replace a Client Profile object.
*AllApi* | [**replace_msg_vpn_client_username**](docs/AllApi.md#replace_msg_vpn_client_username) | **Put** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Replace a Client Username object.
*AllApi* | [**replace_msg_vpn_distributed_cache**](docs/AllApi.md#replace_msg_vpn_distributed_cache) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Replace a Distributed Cache object.
*AllApi* | [**replace_msg_vpn_distributed_cache_cluster**](docs/AllApi.md#replace_msg_vpn_distributed_cache_cluster) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Replace a Cache Cluster object.
*AllApi* | [**replace_msg_vpn_distributed_cache_cluster_instance**](docs/AllApi.md#replace_msg_vpn_distributed_cache_cluster_instance) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Replace a Cache Instance object.
*AllApi* | [**replace_msg_vpn_dmr_bridge**](docs/AllApi.md#replace_msg_vpn_dmr_bridge) | **Put** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Replace a DMR Bridge object.
*AllApi* | [**replace_msg_vpn_jndi_connection_factory**](docs/AllApi.md#replace_msg_vpn_jndi_connection_factory) | **Put** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Replace a JNDI Connection Factory object.
*AllApi* | [**replace_msg_vpn_jndi_queue**](docs/AllApi.md#replace_msg_vpn_jndi_queue) | **Put** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Replace a JNDI Queue object.
*AllApi* | [**replace_msg_vpn_jndi_topic**](docs/AllApi.md#replace_msg_vpn_jndi_topic) | **Put** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Replace a JNDI Topic object.
*AllApi* | [**replace_msg_vpn_mqtt_retain_cache**](docs/AllApi.md#replace_msg_vpn_mqtt_retain_cache) | **Put** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Replace an MQTT Retain Cache object.
*AllApi* | [**replace_msg_vpn_mqtt_session**](docs/AllApi.md#replace_msg_vpn_mqtt_session) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Replace an MQTT Session object.
*AllApi* | [**replace_msg_vpn_mqtt_session_subscription**](docs/AllApi.md#replace_msg_vpn_mqtt_session_subscription) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Replace a Subscription object.
*AllApi* | [**replace_msg_vpn_queue**](docs/AllApi.md#replace_msg_vpn_queue) | **Put** /msgVpns/{msgVpnName}/queues/{queueName} | Replace a Queue object.
*AllApi* | [**replace_msg_vpn_queue_template**](docs/AllApi.md#replace_msg_vpn_queue_template) | **Put** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Replace a Queue Template object.
*AllApi* | [**replace_msg_vpn_replay_log**](docs/AllApi.md#replace_msg_vpn_replay_log) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Replace a Replay Log object.
*AllApi* | [**replace_msg_vpn_replicated_topic**](docs/AllApi.md#replace_msg_vpn_replicated_topic) | **Put** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Replace a Replicated Topic object.
*AllApi* | [**replace_msg_vpn_rest_delivery_point**](docs/AllApi.md#replace_msg_vpn_rest_delivery_point) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Replace a REST Delivery Point object.
*AllApi* | [**replace_msg_vpn_rest_delivery_point_queue_binding**](docs/AllApi.md#replace_msg_vpn_rest_delivery_point_queue_binding) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Replace a Queue Binding object.
*AllApi* | [**replace_msg_vpn_rest_delivery_point_rest_consumer**](docs/AllApi.md#replace_msg_vpn_rest_delivery_point_rest_consumer) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Replace a REST Consumer object.
*AllApi* | [**replace_msg_vpn_topic_endpoint**](docs/AllApi.md#replace_msg_vpn_topic_endpoint) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Replace a Topic Endpoint object.
*AllApi* | [**replace_msg_vpn_topic_endpoint_template**](docs/AllApi.md#replace_msg_vpn_topic_endpoint_template) | **Put** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Replace a Topic Endpoint Template object.
*AllApi* | [**replace_virtual_hostname**](docs/AllApi.md#replace_virtual_hostname) | **Put** /virtualHostnames/{virtualHostname} | Replace a Virtual Hostname object.
*AllApi* | [**update_broker**](docs/AllApi.md#update_broker) | **Patch** / | Update a Broker object.
*AllApi* | [**update_cert_authority**](docs/AllApi.md#update_cert_authority) | **Patch** /certAuthorities/{certAuthorityName} | Update a Certificate Authority object.
*AllApi* | [**update_client_cert_authority**](docs/AllApi.md#update_client_cert_authority) | **Patch** /clientCertAuthorities/{certAuthorityName} | Update a Client Certificate Authority object.
*AllApi* | [**update_dmr_cluster**](docs/AllApi.md#update_dmr_cluster) | **Patch** /dmrClusters/{dmrClusterName} | Update a Cluster object.
*AllApi* | [**update_dmr_cluster_link**](docs/AllApi.md#update_dmr_cluster_link) | **Patch** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Update a Link object.
*AllApi* | [**update_domain_cert_authority**](docs/AllApi.md#update_domain_cert_authority) | **Patch** /domainCertAuthorities/{certAuthorityName} | Update a Domain Certificate Authority object.
*AllApi* | [**update_msg_vpn**](docs/AllApi.md#update_msg_vpn) | **Patch** /msgVpns/{msgVpnName} | Update a Message VPN object.
*AllApi* | [**update_msg_vpn_acl_profile**](docs/AllApi.md#update_msg_vpn_acl_profile) | **Patch** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Update an ACL Profile object.
*AllApi* | [**update_msg_vpn_authentication_oauth_provider**](docs/AllApi.md#update_msg_vpn_authentication_oauth_provider) | **Patch** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Update an OAuth Provider object.
*AllApi* | [**update_msg_vpn_authorization_group**](docs/AllApi.md#update_msg_vpn_authorization_group) | **Patch** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Update an LDAP Authorization Group object.
*AllApi* | [**update_msg_vpn_bridge**](docs/AllApi.md#update_msg_vpn_bridge) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Update a Bridge object.
*AllApi* | [**update_msg_vpn_bridge_remote_msg_vpn**](docs/AllApi.md#update_msg_vpn_bridge_remote_msg_vpn) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Update a Remote Message VPN object.
*AllApi* | [**update_msg_vpn_client_profile**](docs/AllApi.md#update_msg_vpn_client_profile) | **Patch** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Update a Client Profile object.
*AllApi* | [**update_msg_vpn_client_username**](docs/AllApi.md#update_msg_vpn_client_username) | **Patch** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Update a Client Username object.
*AllApi* | [**update_msg_vpn_distributed_cache**](docs/AllApi.md#update_msg_vpn_distributed_cache) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Update a Distributed Cache object.
*AllApi* | [**update_msg_vpn_distributed_cache_cluster**](docs/AllApi.md#update_msg_vpn_distributed_cache_cluster) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Update a Cache Cluster object.
*AllApi* | [**update_msg_vpn_distributed_cache_cluster_instance**](docs/AllApi.md#update_msg_vpn_distributed_cache_cluster_instance) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Update a Cache Instance object.
*AllApi* | [**update_msg_vpn_dmr_bridge**](docs/AllApi.md#update_msg_vpn_dmr_bridge) | **Patch** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Update a DMR Bridge object.
*AllApi* | [**update_msg_vpn_jndi_connection_factory**](docs/AllApi.md#update_msg_vpn_jndi_connection_factory) | **Patch** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Update a JNDI Connection Factory object.
*AllApi* | [**update_msg_vpn_jndi_queue**](docs/AllApi.md#update_msg_vpn_jndi_queue) | **Patch** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Update a JNDI Queue object.
*AllApi* | [**update_msg_vpn_jndi_topic**](docs/AllApi.md#update_msg_vpn_jndi_topic) | **Patch** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Update a JNDI Topic object.
*AllApi* | [**update_msg_vpn_mqtt_retain_cache**](docs/AllApi.md#update_msg_vpn_mqtt_retain_cache) | **Patch** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Update an MQTT Retain Cache object.
*AllApi* | [**update_msg_vpn_mqtt_session**](docs/AllApi.md#update_msg_vpn_mqtt_session) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Update an MQTT Session object.
*AllApi* | [**update_msg_vpn_mqtt_session_subscription**](docs/AllApi.md#update_msg_vpn_mqtt_session_subscription) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Update a Subscription object.
*AllApi* | [**update_msg_vpn_queue**](docs/AllApi.md#update_msg_vpn_queue) | **Patch** /msgVpns/{msgVpnName}/queues/{queueName} | Update a Queue object.
*AllApi* | [**update_msg_vpn_queue_template**](docs/AllApi.md#update_msg_vpn_queue_template) | **Patch** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Update a Queue Template object.
*AllApi* | [**update_msg_vpn_replay_log**](docs/AllApi.md#update_msg_vpn_replay_log) | **Patch** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Update a Replay Log object.
*AllApi* | [**update_msg_vpn_replicated_topic**](docs/AllApi.md#update_msg_vpn_replicated_topic) | **Patch** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Update a Replicated Topic object.
*AllApi* | [**update_msg_vpn_rest_delivery_point**](docs/AllApi.md#update_msg_vpn_rest_delivery_point) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Update a REST Delivery Point object.
*AllApi* | [**update_msg_vpn_rest_delivery_point_queue_binding**](docs/AllApi.md#update_msg_vpn_rest_delivery_point_queue_binding) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Update a Queue Binding object.
*AllApi* | [**update_msg_vpn_rest_delivery_point_rest_consumer**](docs/AllApi.md#update_msg_vpn_rest_delivery_point_rest_consumer) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Update a REST Consumer object.
*AllApi* | [**update_msg_vpn_topic_endpoint**](docs/AllApi.md#update_msg_vpn_topic_endpoint) | **Patch** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Update a Topic Endpoint object.
*AllApi* | [**update_msg_vpn_topic_endpoint_template**](docs/AllApi.md#update_msg_vpn_topic_endpoint_template) | **Patch** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Update a Topic Endpoint Template object.
*AllApi* | [**update_virtual_hostname**](docs/AllApi.md#update_virtual_hostname) | **Patch** /virtualHostnames/{virtualHostname} | Update a Virtual Hostname object.
*AuthenticationOauthProviderApi* | [**create_msg_vpn_authentication_oauth_provider**](docs/AuthenticationOauthProviderApi.md#create_msg_vpn_authentication_oauth_provider) | **Post** /msgVpns/{msgVpnName}/authenticationOauthProviders | Create an OAuth Provider object.
*AuthenticationOauthProviderApi* | [**delete_msg_vpn_authentication_oauth_provider**](docs/AuthenticationOauthProviderApi.md#delete_msg_vpn_authentication_oauth_provider) | **Delete** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Delete an OAuth Provider object.
*AuthenticationOauthProviderApi* | [**get_msg_vpn_authentication_oauth_provider**](docs/AuthenticationOauthProviderApi.md#get_msg_vpn_authentication_oauth_provider) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Get an OAuth Provider object.
*AuthenticationOauthProviderApi* | [**get_msg_vpn_authentication_oauth_providers**](docs/AuthenticationOauthProviderApi.md#get_msg_vpn_authentication_oauth_providers) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders | Get a list of OAuth Provider objects.
*AuthenticationOauthProviderApi* | [**replace_msg_vpn_authentication_oauth_provider**](docs/AuthenticationOauthProviderApi.md#replace_msg_vpn_authentication_oauth_provider) | **Put** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Replace an OAuth Provider object.
*AuthenticationOauthProviderApi* | [**update_msg_vpn_authentication_oauth_provider**](docs/AuthenticationOauthProviderApi.md#update_msg_vpn_authentication_oauth_provider) | **Patch** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Update an OAuth Provider object.
*AuthorizationGroupApi* | [**create_msg_vpn_authorization_group**](docs/AuthorizationGroupApi.md#create_msg_vpn_authorization_group) | **Post** /msgVpns/{msgVpnName}/authorizationGroups | Create an LDAP Authorization Group object.
*AuthorizationGroupApi* | [**delete_msg_vpn_authorization_group**](docs/AuthorizationGroupApi.md#delete_msg_vpn_authorization_group) | **Delete** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Delete an LDAP Authorization Group object.
*AuthorizationGroupApi* | [**get_msg_vpn_authorization_group**](docs/AuthorizationGroupApi.md#get_msg_vpn_authorization_group) | **Get** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Get an LDAP Authorization Group object.
*AuthorizationGroupApi* | [**get_msg_vpn_authorization_groups**](docs/AuthorizationGroupApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Get a list of LDAP Authorization Group objects.
*AuthorizationGroupApi* | [**replace_msg_vpn_authorization_group**](docs/AuthorizationGroupApi.md#replace_msg_vpn_authorization_group) | **Put** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Replace an LDAP Authorization Group object.
*AuthorizationGroupApi* | [**update_msg_vpn_authorization_group**](docs/AuthorizationGroupApi.md#update_msg_vpn_authorization_group) | **Patch** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Update an LDAP Authorization Group object.
*BridgeApi* | [**create_msg_vpn_bridge**](docs/BridgeApi.md#create_msg_vpn_bridge) | **Post** /msgVpns/{msgVpnName}/bridges | Create a Bridge object.
*BridgeApi* | [**create_msg_vpn_bridge_remote_msg_vpn**](docs/BridgeApi.md#create_msg_vpn_bridge_remote_msg_vpn) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Create a Remote Message VPN object.
*BridgeApi* | [**create_msg_vpn_bridge_remote_subscription**](docs/BridgeApi.md#create_msg_vpn_bridge_remote_subscription) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Create a Remote Subscription object.
*BridgeApi* | [**create_msg_vpn_bridge_tls_trusted_common_name**](docs/BridgeApi.md#create_msg_vpn_bridge_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*BridgeApi* | [**delete_msg_vpn_bridge**](docs/BridgeApi.md#delete_msg_vpn_bridge) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Delete a Bridge object.
*BridgeApi* | [**delete_msg_vpn_bridge_remote_msg_vpn**](docs/BridgeApi.md#delete_msg_vpn_bridge_remote_msg_vpn) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Delete a Remote Message VPN object.
*BridgeApi* | [**delete_msg_vpn_bridge_remote_subscription**](docs/BridgeApi.md#delete_msg_vpn_bridge_remote_subscription) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Delete a Remote Subscription object.
*BridgeApi* | [**delete_msg_vpn_bridge_tls_trusted_common_name**](docs/BridgeApi.md#delete_msg_vpn_bridge_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*BridgeApi* | [**get_msg_vpn_bridge**](docs/BridgeApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
*BridgeApi* | [**get_msg_vpn_bridge_remote_msg_vpn**](docs/BridgeApi.md#get_msg_vpn_bridge_remote_msg_vpn) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Get a Remote Message VPN object.
*BridgeApi* | [**get_msg_vpn_bridge_remote_msg_vpns**](docs/BridgeApi.md#get_msg_vpn_bridge_remote_msg_vpns) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Get a list of Remote Message VPN objects.
*BridgeApi* | [**get_msg_vpn_bridge_remote_subscription**](docs/BridgeApi.md#get_msg_vpn_bridge_remote_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Get a Remote Subscription object.
*BridgeApi* | [**get_msg_vpn_bridge_remote_subscriptions**](docs/BridgeApi.md#get_msg_vpn_bridge_remote_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Get a list of Remote Subscription objects.
*BridgeApi* | [**get_msg_vpn_bridge_tls_trusted_common_name**](docs/BridgeApi.md#get_msg_vpn_bridge_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*BridgeApi* | [**get_msg_vpn_bridge_tls_trusted_common_names**](docs/BridgeApi.md#get_msg_vpn_bridge_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*BridgeApi* | [**get_msg_vpn_bridges**](docs/BridgeApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*BridgeApi* | [**replace_msg_vpn_bridge**](docs/BridgeApi.md#replace_msg_vpn_bridge) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Replace a Bridge object.
*BridgeApi* | [**replace_msg_vpn_bridge_remote_msg_vpn**](docs/BridgeApi.md#replace_msg_vpn_bridge_remote_msg_vpn) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Replace a Remote Message VPN object.
*BridgeApi* | [**update_msg_vpn_bridge**](docs/BridgeApi.md#update_msg_vpn_bridge) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Update a Bridge object.
*BridgeApi* | [**update_msg_vpn_bridge_remote_msg_vpn**](docs/BridgeApi.md#update_msg_vpn_bridge_remote_msg_vpn) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Update a Remote Message VPN object.
*CertAuthorityApi* | [**create_cert_authority**](docs/CertAuthorityApi.md#create_cert_authority) | **Post** /certAuthorities | Create a Certificate Authority object.
*CertAuthorityApi* | [**create_cert_authority_ocsp_tls_trusted_common_name**](docs/CertAuthorityApi.md#create_cert_authority_ocsp_tls_trusted_common_name) | **Post** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Create an OCSP Responder Trusted Common Name object.
*CertAuthorityApi* | [**delete_cert_authority**](docs/CertAuthorityApi.md#delete_cert_authority) | **Delete** /certAuthorities/{certAuthorityName} | Delete a Certificate Authority object.
*CertAuthorityApi* | [**delete_cert_authority_ocsp_tls_trusted_common_name**](docs/CertAuthorityApi.md#delete_cert_authority_ocsp_tls_trusted_common_name) | **Delete** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Delete an OCSP Responder Trusted Common Name object.
*CertAuthorityApi* | [**get_cert_authorities**](docs/CertAuthorityApi.md#get_cert_authorities) | **Get** /certAuthorities | Get a list of Certificate Authority objects.
*CertAuthorityApi* | [**get_cert_authority**](docs/CertAuthorityApi.md#get_cert_authority) | **Get** /certAuthorities/{certAuthorityName} | Get a Certificate Authority object.
*CertAuthorityApi* | [**get_cert_authority_ocsp_tls_trusted_common_name**](docs/CertAuthorityApi.md#get_cert_authority_ocsp_tls_trusted_common_name) | **Get** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Get an OCSP Responder Trusted Common Name object.
*CertAuthorityApi* | [**get_cert_authority_ocsp_tls_trusted_common_names**](docs/CertAuthorityApi.md#get_cert_authority_ocsp_tls_trusted_common_names) | **Get** /certAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Get a list of OCSP Responder Trusted Common Name objects.
*CertAuthorityApi* | [**replace_cert_authority**](docs/CertAuthorityApi.md#replace_cert_authority) | **Put** /certAuthorities/{certAuthorityName} | Replace a Certificate Authority object.
*CertAuthorityApi* | [**update_cert_authority**](docs/CertAuthorityApi.md#update_cert_authority) | **Patch** /certAuthorities/{certAuthorityName} | Update a Certificate Authority object.
*ClientCertAuthorityApi* | [**create_client_cert_authority**](docs/ClientCertAuthorityApi.md#create_client_cert_authority) | **Post** /clientCertAuthorities | Create a Client Certificate Authority object.
*ClientCertAuthorityApi* | [**create_client_cert_authority_ocsp_tls_trusted_common_name**](docs/ClientCertAuthorityApi.md#create_client_cert_authority_ocsp_tls_trusted_common_name) | **Post** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Create an OCSP Responder Trusted Common Name object.
*ClientCertAuthorityApi* | [**delete_client_cert_authority**](docs/ClientCertAuthorityApi.md#delete_client_cert_authority) | **Delete** /clientCertAuthorities/{certAuthorityName} | Delete a Client Certificate Authority object.
*ClientCertAuthorityApi* | [**delete_client_cert_authority_ocsp_tls_trusted_common_name**](docs/ClientCertAuthorityApi.md#delete_client_cert_authority_ocsp_tls_trusted_common_name) | **Delete** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Delete an OCSP Responder Trusted Common Name object.
*ClientCertAuthorityApi* | [**get_client_cert_authorities**](docs/ClientCertAuthorityApi.md#get_client_cert_authorities) | **Get** /clientCertAuthorities | Get a list of Client Certificate Authority objects.
*ClientCertAuthorityApi* | [**get_client_cert_authority**](docs/ClientCertAuthorityApi.md#get_client_cert_authority) | **Get** /clientCertAuthorities/{certAuthorityName} | Get a Client Certificate Authority object.
*ClientCertAuthorityApi* | [**get_client_cert_authority_ocsp_tls_trusted_common_name**](docs/ClientCertAuthorityApi.md#get_client_cert_authority_ocsp_tls_trusted_common_name) | **Get** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames/{ocspTlsTrustedCommonName} | Get an OCSP Responder Trusted Common Name object.
*ClientCertAuthorityApi* | [**get_client_cert_authority_ocsp_tls_trusted_common_names**](docs/ClientCertAuthorityApi.md#get_client_cert_authority_ocsp_tls_trusted_common_names) | **Get** /clientCertAuthorities/{certAuthorityName}/ocspTlsTrustedCommonNames | Get a list of OCSP Responder Trusted Common Name objects.
*ClientCertAuthorityApi* | [**replace_client_cert_authority**](docs/ClientCertAuthorityApi.md#replace_client_cert_authority) | **Put** /clientCertAuthorities/{certAuthorityName} | Replace a Client Certificate Authority object.
*ClientCertAuthorityApi* | [**update_client_cert_authority**](docs/ClientCertAuthorityApi.md#update_client_cert_authority) | **Patch** /clientCertAuthorities/{certAuthorityName} | Update a Client Certificate Authority object.
*ClientProfileApi* | [**create_msg_vpn_client_profile**](docs/ClientProfileApi.md#create_msg_vpn_client_profile) | **Post** /msgVpns/{msgVpnName}/clientProfiles | Create a Client Profile object.
*ClientProfileApi* | [**delete_msg_vpn_client_profile**](docs/ClientProfileApi.md#delete_msg_vpn_client_profile) | **Delete** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Delete a Client Profile object.
*ClientProfileApi* | [**get_msg_vpn_client_profile**](docs/ClientProfileApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Get a Client Profile object.
*ClientProfileApi* | [**get_msg_vpn_client_profiles**](docs/ClientProfileApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Get a list of Client Profile objects.
*ClientProfileApi* | [**replace_msg_vpn_client_profile**](docs/ClientProfileApi.md#replace_msg_vpn_client_profile) | **Put** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Replace a Client Profile object.
*ClientProfileApi* | [**update_msg_vpn_client_profile**](docs/ClientProfileApi.md#update_msg_vpn_client_profile) | **Patch** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Update a Client Profile object.
*ClientUsernameApi* | [**create_msg_vpn_client_username**](docs/ClientUsernameApi.md#create_msg_vpn_client_username) | **Post** /msgVpns/{msgVpnName}/clientUsernames | Create a Client Username object.
*ClientUsernameApi* | [**delete_msg_vpn_client_username**](docs/ClientUsernameApi.md#delete_msg_vpn_client_username) | **Delete** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Delete a Client Username object.
*ClientUsernameApi* | [**get_msg_vpn_client_username**](docs/ClientUsernameApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Get a Client Username object.
*ClientUsernameApi* | [**get_msg_vpn_client_usernames**](docs/ClientUsernameApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Get a list of Client Username objects.
*ClientUsernameApi* | [**replace_msg_vpn_client_username**](docs/ClientUsernameApi.md#replace_msg_vpn_client_username) | **Put** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Replace a Client Username object.
*ClientUsernameApi* | [**update_msg_vpn_client_username**](docs/ClientUsernameApi.md#update_msg_vpn_client_username) | **Patch** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Update a Client Username object.
*DistributedCacheApi* | [**create_msg_vpn_distributed_cache**](docs/DistributedCacheApi.md#create_msg_vpn_distributed_cache) | **Post** /msgVpns/{msgVpnName}/distributedCaches | Create a Distributed Cache object.
*DistributedCacheApi* | [**create_msg_vpn_distributed_cache_cluster**](docs/DistributedCacheApi.md#create_msg_vpn_distributed_cache_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Create a Cache Cluster object.
*DistributedCacheApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/DistributedCacheApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Create a Home Cache Cluster object.
*DistributedCacheApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/DistributedCacheApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Create a Topic Prefix object.
*DistributedCacheApi* | [**create_msg_vpn_distributed_cache_cluster_instance**](docs/DistributedCacheApi.md#create_msg_vpn_distributed_cache_cluster_instance) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Create a Cache Instance object.
*DistributedCacheApi* | [**create_msg_vpn_distributed_cache_cluster_topic**](docs/DistributedCacheApi.md#create_msg_vpn_distributed_cache_cluster_topic) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Create a Topic object.
*DistributedCacheApi* | [**delete_msg_vpn_distributed_cache**](docs/DistributedCacheApi.md#delete_msg_vpn_distributed_cache) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Delete a Distributed Cache object.
*DistributedCacheApi* | [**delete_msg_vpn_distributed_cache_cluster**](docs/DistributedCacheApi.md#delete_msg_vpn_distributed_cache_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Delete a Cache Cluster object.
*DistributedCacheApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/DistributedCacheApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Delete a Home Cache Cluster object.
*DistributedCacheApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/DistributedCacheApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Delete a Topic Prefix object.
*DistributedCacheApi* | [**delete_msg_vpn_distributed_cache_cluster_instance**](docs/DistributedCacheApi.md#delete_msg_vpn_distributed_cache_cluster_instance) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Delete a Cache Instance object.
*DistributedCacheApi* | [**delete_msg_vpn_distributed_cache_cluster_topic**](docs/DistributedCacheApi.md#delete_msg_vpn_distributed_cache_cluster_topic) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Delete a Topic object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Get a Home Cache Cluster object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Get a Topic Prefix object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Get a list of Topic Prefix objects.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Get a list of Home Cache Cluster objects.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_instance**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_instances**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_topic**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_topic) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Get a Topic object.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_cluster_topics**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_cluster_topics) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Get a list of Topic objects.
*DistributedCacheApi* | [**get_msg_vpn_distributed_cache_clusters**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
*DistributedCacheApi* | [**get_msg_vpn_distributed_caches**](docs/DistributedCacheApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*DistributedCacheApi* | [**replace_msg_vpn_distributed_cache**](docs/DistributedCacheApi.md#replace_msg_vpn_distributed_cache) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Replace a Distributed Cache object.
*DistributedCacheApi* | [**replace_msg_vpn_distributed_cache_cluster**](docs/DistributedCacheApi.md#replace_msg_vpn_distributed_cache_cluster) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Replace a Cache Cluster object.
*DistributedCacheApi* | [**replace_msg_vpn_distributed_cache_cluster_instance**](docs/DistributedCacheApi.md#replace_msg_vpn_distributed_cache_cluster_instance) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Replace a Cache Instance object.
*DistributedCacheApi* | [**update_msg_vpn_distributed_cache**](docs/DistributedCacheApi.md#update_msg_vpn_distributed_cache) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Update a Distributed Cache object.
*DistributedCacheApi* | [**update_msg_vpn_distributed_cache_cluster**](docs/DistributedCacheApi.md#update_msg_vpn_distributed_cache_cluster) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Update a Cache Cluster object.
*DistributedCacheApi* | [**update_msg_vpn_distributed_cache_cluster_instance**](docs/DistributedCacheApi.md#update_msg_vpn_distributed_cache_cluster_instance) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Update a Cache Instance object.
*DmrBridgeApi* | [**create_msg_vpn_dmr_bridge**](docs/DmrBridgeApi.md#create_msg_vpn_dmr_bridge) | **Post** /msgVpns/{msgVpnName}/dmrBridges | Create a DMR Bridge object.
*DmrBridgeApi* | [**delete_msg_vpn_dmr_bridge**](docs/DmrBridgeApi.md#delete_msg_vpn_dmr_bridge) | **Delete** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Delete a DMR Bridge object.
*DmrBridgeApi* | [**get_msg_vpn_dmr_bridge**](docs/DmrBridgeApi.md#get_msg_vpn_dmr_bridge) | **Get** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Get a DMR Bridge object.
*DmrBridgeApi* | [**get_msg_vpn_dmr_bridges**](docs/DmrBridgeApi.md#get_msg_vpn_dmr_bridges) | **Get** /msgVpns/{msgVpnName}/dmrBridges | Get a list of DMR Bridge objects.
*DmrBridgeApi* | [**replace_msg_vpn_dmr_bridge**](docs/DmrBridgeApi.md#replace_msg_vpn_dmr_bridge) | **Put** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Replace a DMR Bridge object.
*DmrBridgeApi* | [**update_msg_vpn_dmr_bridge**](docs/DmrBridgeApi.md#update_msg_vpn_dmr_bridge) | **Patch** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Update a DMR Bridge object.
*DmrClusterApi* | [**create_dmr_cluster**](docs/DmrClusterApi.md#create_dmr_cluster) | **Post** /dmrClusters | Create a Cluster object.
*DmrClusterApi* | [**create_dmr_cluster_link**](docs/DmrClusterApi.md#create_dmr_cluster_link) | **Post** /dmrClusters/{dmrClusterName}/links | Create a Link object.
*DmrClusterApi* | [**create_dmr_cluster_link_remote_address**](docs/DmrClusterApi.md#create_dmr_cluster_link_remote_address) | **Post** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Create a Remote Address object.
*DmrClusterApi* | [**create_dmr_cluster_link_tls_trusted_common_name**](docs/DmrClusterApi.md#create_dmr_cluster_link_tls_trusted_common_name) | **Post** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*DmrClusterApi* | [**delete_dmr_cluster**](docs/DmrClusterApi.md#delete_dmr_cluster) | **Delete** /dmrClusters/{dmrClusterName} | Delete a Cluster object.
*DmrClusterApi* | [**delete_dmr_cluster_link**](docs/DmrClusterApi.md#delete_dmr_cluster_link) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Delete a Link object.
*DmrClusterApi* | [**delete_dmr_cluster_link_remote_address**](docs/DmrClusterApi.md#delete_dmr_cluster_link_remote_address) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Delete a Remote Address object.
*DmrClusterApi* | [**delete_dmr_cluster_link_tls_trusted_common_name**](docs/DmrClusterApi.md#delete_dmr_cluster_link_tls_trusted_common_name) | **Delete** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*DmrClusterApi* | [**get_dmr_cluster**](docs/DmrClusterApi.md#get_dmr_cluster) | **Get** /dmrClusters/{dmrClusterName} | Get a Cluster object.
*DmrClusterApi* | [**get_dmr_cluster_link**](docs/DmrClusterApi.md#get_dmr_cluster_link) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Get a Link object.
*DmrClusterApi* | [**get_dmr_cluster_link_remote_address**](docs/DmrClusterApi.md#get_dmr_cluster_link_remote_address) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses/{remoteAddress} | Get a Remote Address object.
*DmrClusterApi* | [**get_dmr_cluster_link_remote_addresses**](docs/DmrClusterApi.md#get_dmr_cluster_link_remote_addresses) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/remoteAddresses | Get a list of Remote Address objects.
*DmrClusterApi* | [**get_dmr_cluster_link_tls_trusted_common_name**](docs/DmrClusterApi.md#get_dmr_cluster_link_tls_trusted_common_name) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*DmrClusterApi* | [**get_dmr_cluster_link_tls_trusted_common_names**](docs/DmrClusterApi.md#get_dmr_cluster_link_tls_trusted_common_names) | **Get** /dmrClusters/{dmrClusterName}/links/{remoteNodeName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*DmrClusterApi* | [**get_dmr_cluster_links**](docs/DmrClusterApi.md#get_dmr_cluster_links) | **Get** /dmrClusters/{dmrClusterName}/links | Get a list of Link objects.
*DmrClusterApi* | [**get_dmr_clusters**](docs/DmrClusterApi.md#get_dmr_clusters) | **Get** /dmrClusters | Get a list of Cluster objects.
*DmrClusterApi* | [**replace_dmr_cluster**](docs/DmrClusterApi.md#replace_dmr_cluster) | **Put** /dmrClusters/{dmrClusterName} | Replace a Cluster object.
*DmrClusterApi* | [**replace_dmr_cluster_link**](docs/DmrClusterApi.md#replace_dmr_cluster_link) | **Put** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Replace a Link object.
*DmrClusterApi* | [**update_dmr_cluster**](docs/DmrClusterApi.md#update_dmr_cluster) | **Patch** /dmrClusters/{dmrClusterName} | Update a Cluster object.
*DmrClusterApi* | [**update_dmr_cluster_link**](docs/DmrClusterApi.md#update_dmr_cluster_link) | **Patch** /dmrClusters/{dmrClusterName}/links/{remoteNodeName} | Update a Link object.
*DomainCertAuthorityApi* | [**create_domain_cert_authority**](docs/DomainCertAuthorityApi.md#create_domain_cert_authority) | **Post** /domainCertAuthorities | Create a Domain Certificate Authority object.
*DomainCertAuthorityApi* | [**delete_domain_cert_authority**](docs/DomainCertAuthorityApi.md#delete_domain_cert_authority) | **Delete** /domainCertAuthorities/{certAuthorityName} | Delete a Domain Certificate Authority object.
*DomainCertAuthorityApi* | [**get_domain_cert_authorities**](docs/DomainCertAuthorityApi.md#get_domain_cert_authorities) | **Get** /domainCertAuthorities | Get a list of Domain Certificate Authority objects.
*DomainCertAuthorityApi* | [**get_domain_cert_authority**](docs/DomainCertAuthorityApi.md#get_domain_cert_authority) | **Get** /domainCertAuthorities/{certAuthorityName} | Get a Domain Certificate Authority object.
*DomainCertAuthorityApi* | [**replace_domain_cert_authority**](docs/DomainCertAuthorityApi.md#replace_domain_cert_authority) | **Put** /domainCertAuthorities/{certAuthorityName} | Replace a Domain Certificate Authority object.
*DomainCertAuthorityApi* | [**update_domain_cert_authority**](docs/DomainCertAuthorityApi.md#update_domain_cert_authority) | **Patch** /domainCertAuthorities/{certAuthorityName} | Update a Domain Certificate Authority object.
*JndiApi* | [**create_msg_vpn_jndi_connection_factory**](docs/JndiApi.md#create_msg_vpn_jndi_connection_factory) | **Post** /msgVpns/{msgVpnName}/jndiConnectionFactories | Create a JNDI Connection Factory object.
*JndiApi* | [**create_msg_vpn_jndi_queue**](docs/JndiApi.md#create_msg_vpn_jndi_queue) | **Post** /msgVpns/{msgVpnName}/jndiQueues | Create a JNDI Queue object.
*JndiApi* | [**create_msg_vpn_jndi_topic**](docs/JndiApi.md#create_msg_vpn_jndi_topic) | **Post** /msgVpns/{msgVpnName}/jndiTopics | Create a JNDI Topic object.
*JndiApi* | [**delete_msg_vpn_jndi_connection_factory**](docs/JndiApi.md#delete_msg_vpn_jndi_connection_factory) | **Delete** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Delete a JNDI Connection Factory object.
*JndiApi* | [**delete_msg_vpn_jndi_queue**](docs/JndiApi.md#delete_msg_vpn_jndi_queue) | **Delete** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Delete a JNDI Queue object.
*JndiApi* | [**delete_msg_vpn_jndi_topic**](docs/JndiApi.md#delete_msg_vpn_jndi_topic) | **Delete** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Delete a JNDI Topic object.
*JndiApi* | [**get_msg_vpn_jndi_connection_factories**](docs/JndiApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Get a list of JNDI Connection Factory objects.
*JndiApi* | [**get_msg_vpn_jndi_connection_factory**](docs/JndiApi.md#get_msg_vpn_jndi_connection_factory) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Get a JNDI Connection Factory object.
*JndiApi* | [**get_msg_vpn_jndi_queue**](docs/JndiApi.md#get_msg_vpn_jndi_queue) | **Get** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Get a JNDI Queue object.
*JndiApi* | [**get_msg_vpn_jndi_queues**](docs/JndiApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Get a list of JNDI Queue objects.
*JndiApi* | [**get_msg_vpn_jndi_topic**](docs/JndiApi.md#get_msg_vpn_jndi_topic) | **Get** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Get a JNDI Topic object.
*JndiApi* | [**get_msg_vpn_jndi_topics**](docs/JndiApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Get a list of JNDI Topic objects.
*JndiApi* | [**replace_msg_vpn_jndi_connection_factory**](docs/JndiApi.md#replace_msg_vpn_jndi_connection_factory) | **Put** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Replace a JNDI Connection Factory object.
*JndiApi* | [**replace_msg_vpn_jndi_queue**](docs/JndiApi.md#replace_msg_vpn_jndi_queue) | **Put** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Replace a JNDI Queue object.
*JndiApi* | [**replace_msg_vpn_jndi_topic**](docs/JndiApi.md#replace_msg_vpn_jndi_topic) | **Put** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Replace a JNDI Topic object.
*JndiApi* | [**update_msg_vpn_jndi_connection_factory**](docs/JndiApi.md#update_msg_vpn_jndi_connection_factory) | **Patch** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Update a JNDI Connection Factory object.
*JndiApi* | [**update_msg_vpn_jndi_queue**](docs/JndiApi.md#update_msg_vpn_jndi_queue) | **Patch** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Update a JNDI Queue object.
*JndiApi* | [**update_msg_vpn_jndi_topic**](docs/JndiApi.md#update_msg_vpn_jndi_topic) | **Patch** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Update a JNDI Topic object.
*MqttRetainCacheApi* | [**create_msg_vpn_mqtt_retain_cache**](docs/MqttRetainCacheApi.md#create_msg_vpn_mqtt_retain_cache) | **Post** /msgVpns/{msgVpnName}/mqttRetainCaches | Create an MQTT Retain Cache object.
*MqttRetainCacheApi* | [**delete_msg_vpn_mqtt_retain_cache**](docs/MqttRetainCacheApi.md#delete_msg_vpn_mqtt_retain_cache) | **Delete** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Delete an MQTT Retain Cache object.
*MqttRetainCacheApi* | [**get_msg_vpn_mqtt_retain_cache**](docs/MqttRetainCacheApi.md#get_msg_vpn_mqtt_retain_cache) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Get an MQTT Retain Cache object.
*MqttRetainCacheApi* | [**get_msg_vpn_mqtt_retain_caches**](docs/MqttRetainCacheApi.md#get_msg_vpn_mqtt_retain_caches) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches | Get a list of MQTT Retain Cache objects.
*MqttRetainCacheApi* | [**replace_msg_vpn_mqtt_retain_cache**](docs/MqttRetainCacheApi.md#replace_msg_vpn_mqtt_retain_cache) | **Put** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Replace an MQTT Retain Cache object.
*MqttRetainCacheApi* | [**update_msg_vpn_mqtt_retain_cache**](docs/MqttRetainCacheApi.md#update_msg_vpn_mqtt_retain_cache) | **Patch** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Update an MQTT Retain Cache object.
*MqttSessionApi* | [**create_msg_vpn_mqtt_session**](docs/MqttSessionApi.md#create_msg_vpn_mqtt_session) | **Post** /msgVpns/{msgVpnName}/mqttSessions | Create an MQTT Session object.
*MqttSessionApi* | [**create_msg_vpn_mqtt_session_subscription**](docs/MqttSessionApi.md#create_msg_vpn_mqtt_session_subscription) | **Post** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Create a Subscription object.
*MqttSessionApi* | [**delete_msg_vpn_mqtt_session**](docs/MqttSessionApi.md#delete_msg_vpn_mqtt_session) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Delete an MQTT Session object.
*MqttSessionApi* | [**delete_msg_vpn_mqtt_session_subscription**](docs/MqttSessionApi.md#delete_msg_vpn_mqtt_session_subscription) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Delete a Subscription object.
*MqttSessionApi* | [**get_msg_vpn_mqtt_session**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
*MqttSessionApi* | [**get_msg_vpn_mqtt_session_subscription**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_session_subscription) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Get a Subscription object.
*MqttSessionApi* | [**get_msg_vpn_mqtt_session_subscriptions**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_session_subscriptions) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Get a list of Subscription objects.
*MqttSessionApi* | [**get_msg_vpn_mqtt_sessions**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*MqttSessionApi* | [**replace_msg_vpn_mqtt_session**](docs/MqttSessionApi.md#replace_msg_vpn_mqtt_session) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Replace an MQTT Session object.
*MqttSessionApi* | [**replace_msg_vpn_mqtt_session_subscription**](docs/MqttSessionApi.md#replace_msg_vpn_mqtt_session_subscription) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Replace a Subscription object.
*MqttSessionApi* | [**update_msg_vpn_mqtt_session**](docs/MqttSessionApi.md#update_msg_vpn_mqtt_session) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Update an MQTT Session object.
*MqttSessionApi* | [**update_msg_vpn_mqtt_session_subscription**](docs/MqttSessionApi.md#update_msg_vpn_mqtt_session_subscription) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Update a Subscription object.
*MsgVpnApi* | [**create_msg_vpn**](docs/MsgVpnApi.md#create_msg_vpn) | **Post** /msgVpns | Create a Message VPN object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile) | **Post** /msgVpns/{msgVpnName}/aclProfiles | Create an ACL Profile object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile_client_connect_exception**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile_client_connect_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Create a Client Connect Exception object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile_publish_exception**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile_publish_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Create a Publish Topic Exception object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile_publish_topic_exception**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile_publish_topic_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Create a Publish Topic Exception object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile_subscribe_exception**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile_subscribe_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Create a Subscribe Topic Exception object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile_subscribe_share_name_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Create a Subscribe Share Name Exception object.
*MsgVpnApi* | [**create_msg_vpn_acl_profile_subscribe_topic_exception**](docs/MsgVpnApi.md#create_msg_vpn_acl_profile_subscribe_topic_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Create a Subscribe Topic Exception object.
*MsgVpnApi* | [**create_msg_vpn_authentication_oauth_provider**](docs/MsgVpnApi.md#create_msg_vpn_authentication_oauth_provider) | **Post** /msgVpns/{msgVpnName}/authenticationOauthProviders | Create an OAuth Provider object.
*MsgVpnApi* | [**create_msg_vpn_authorization_group**](docs/MsgVpnApi.md#create_msg_vpn_authorization_group) | **Post** /msgVpns/{msgVpnName}/authorizationGroups | Create an LDAP Authorization Group object.
*MsgVpnApi* | [**create_msg_vpn_bridge**](docs/MsgVpnApi.md#create_msg_vpn_bridge) | **Post** /msgVpns/{msgVpnName}/bridges | Create a Bridge object.
*MsgVpnApi* | [**create_msg_vpn_bridge_remote_msg_vpn**](docs/MsgVpnApi.md#create_msg_vpn_bridge_remote_msg_vpn) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Create a Remote Message VPN object.
*MsgVpnApi* | [**create_msg_vpn_bridge_remote_subscription**](docs/MsgVpnApi.md#create_msg_vpn_bridge_remote_subscription) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Create a Remote Subscription object.
*MsgVpnApi* | [**create_msg_vpn_bridge_tls_trusted_common_name**](docs/MsgVpnApi.md#create_msg_vpn_bridge_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*MsgVpnApi* | [**create_msg_vpn_client_profile**](docs/MsgVpnApi.md#create_msg_vpn_client_profile) | **Post** /msgVpns/{msgVpnName}/clientProfiles | Create a Client Profile object.
*MsgVpnApi* | [**create_msg_vpn_client_username**](docs/MsgVpnApi.md#create_msg_vpn_client_username) | **Post** /msgVpns/{msgVpnName}/clientUsernames | Create a Client Username object.
*MsgVpnApi* | [**create_msg_vpn_distributed_cache**](docs/MsgVpnApi.md#create_msg_vpn_distributed_cache) | **Post** /msgVpns/{msgVpnName}/distributedCaches | Create a Distributed Cache object.
*MsgVpnApi* | [**create_msg_vpn_distributed_cache_cluster**](docs/MsgVpnApi.md#create_msg_vpn_distributed_cache_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Create a Cache Cluster object.
*MsgVpnApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/MsgVpnApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Create a Home Cache Cluster object.
*MsgVpnApi* | [**create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/MsgVpnApi.md#create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Create a Topic Prefix object.
*MsgVpnApi* | [**create_msg_vpn_distributed_cache_cluster_instance**](docs/MsgVpnApi.md#create_msg_vpn_distributed_cache_cluster_instance) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Create a Cache Instance object.
*MsgVpnApi* | [**create_msg_vpn_distributed_cache_cluster_topic**](docs/MsgVpnApi.md#create_msg_vpn_distributed_cache_cluster_topic) | **Post** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Create a Topic object.
*MsgVpnApi* | [**create_msg_vpn_dmr_bridge**](docs/MsgVpnApi.md#create_msg_vpn_dmr_bridge) | **Post** /msgVpns/{msgVpnName}/dmrBridges | Create a DMR Bridge object.
*MsgVpnApi* | [**create_msg_vpn_jndi_connection_factory**](docs/MsgVpnApi.md#create_msg_vpn_jndi_connection_factory) | **Post** /msgVpns/{msgVpnName}/jndiConnectionFactories | Create a JNDI Connection Factory object.
*MsgVpnApi* | [**create_msg_vpn_jndi_queue**](docs/MsgVpnApi.md#create_msg_vpn_jndi_queue) | **Post** /msgVpns/{msgVpnName}/jndiQueues | Create a JNDI Queue object.
*MsgVpnApi* | [**create_msg_vpn_jndi_topic**](docs/MsgVpnApi.md#create_msg_vpn_jndi_topic) | **Post** /msgVpns/{msgVpnName}/jndiTopics | Create a JNDI Topic object.
*MsgVpnApi* | [**create_msg_vpn_mqtt_retain_cache**](docs/MsgVpnApi.md#create_msg_vpn_mqtt_retain_cache) | **Post** /msgVpns/{msgVpnName}/mqttRetainCaches | Create an MQTT Retain Cache object.
*MsgVpnApi* | [**create_msg_vpn_mqtt_session**](docs/MsgVpnApi.md#create_msg_vpn_mqtt_session) | **Post** /msgVpns/{msgVpnName}/mqttSessions | Create an MQTT Session object.
*MsgVpnApi* | [**create_msg_vpn_mqtt_session_subscription**](docs/MsgVpnApi.md#create_msg_vpn_mqtt_session_subscription) | **Post** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Create a Subscription object.
*MsgVpnApi* | [**create_msg_vpn_queue**](docs/MsgVpnApi.md#create_msg_vpn_queue) | **Post** /msgVpns/{msgVpnName}/queues | Create a Queue object.
*MsgVpnApi* | [**create_msg_vpn_queue_subscription**](docs/MsgVpnApi.md#create_msg_vpn_queue_subscription) | **Post** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Create a Queue Subscription object.
*MsgVpnApi* | [**create_msg_vpn_queue_template**](docs/MsgVpnApi.md#create_msg_vpn_queue_template) | **Post** /msgVpns/{msgVpnName}/queueTemplates | Create a Queue Template object.
*MsgVpnApi* | [**create_msg_vpn_replay_log**](docs/MsgVpnApi.md#create_msg_vpn_replay_log) | **Post** /msgVpns/{msgVpnName}/replayLogs | Create a Replay Log object.
*MsgVpnApi* | [**create_msg_vpn_replicated_topic**](docs/MsgVpnApi.md#create_msg_vpn_replicated_topic) | **Post** /msgVpns/{msgVpnName}/replicatedTopics | Create a Replicated Topic object.
*MsgVpnApi* | [**create_msg_vpn_rest_delivery_point**](docs/MsgVpnApi.md#create_msg_vpn_rest_delivery_point) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints | Create a REST Delivery Point object.
*MsgVpnApi* | [**create_msg_vpn_rest_delivery_point_queue_binding**](docs/MsgVpnApi.md#create_msg_vpn_rest_delivery_point_queue_binding) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Create a Queue Binding object.
*MsgVpnApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer**](docs/MsgVpnApi.md#create_msg_vpn_rest_delivery_point_rest_consumer) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Create a REST Consumer object.
*MsgVpnApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/MsgVpnApi.md#create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*MsgVpnApi* | [**create_msg_vpn_sequenced_topic**](docs/MsgVpnApi.md#create_msg_vpn_sequenced_topic) | **Post** /msgVpns/{msgVpnName}/sequencedTopics | Create a Sequenced Topic object.
*MsgVpnApi* | [**create_msg_vpn_topic_endpoint**](docs/MsgVpnApi.md#create_msg_vpn_topic_endpoint) | **Post** /msgVpns/{msgVpnName}/topicEndpoints | Create a Topic Endpoint object.
*MsgVpnApi* | [**create_msg_vpn_topic_endpoint_template**](docs/MsgVpnApi.md#create_msg_vpn_topic_endpoint_template) | **Post** /msgVpns/{msgVpnName}/topicEndpointTemplates | Create a Topic Endpoint Template object.
*MsgVpnApi* | [**delete_msg_vpn**](docs/MsgVpnApi.md#delete_msg_vpn) | **Delete** /msgVpns/{msgVpnName} | Delete a Message VPN object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Delete an ACL Profile object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile_client_connect_exception**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile_client_connect_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Delete a Client Connect Exception object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile_publish_exception**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile_publish_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Delete a Publish Topic Exception object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile_publish_topic_exception**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile_publish_topic_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Delete a Publish Topic Exception object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile_subscribe_exception**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile_subscribe_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Delete a Subscribe Topic Exception object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile_subscribe_share_name_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Delete a Subscribe Share Name Exception object.
*MsgVpnApi* | [**delete_msg_vpn_acl_profile_subscribe_topic_exception**](docs/MsgVpnApi.md#delete_msg_vpn_acl_profile_subscribe_topic_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Delete a Subscribe Topic Exception object.
*MsgVpnApi* | [**delete_msg_vpn_authentication_oauth_provider**](docs/MsgVpnApi.md#delete_msg_vpn_authentication_oauth_provider) | **Delete** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Delete an OAuth Provider object.
*MsgVpnApi* | [**delete_msg_vpn_authorization_group**](docs/MsgVpnApi.md#delete_msg_vpn_authorization_group) | **Delete** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Delete an LDAP Authorization Group object.
*MsgVpnApi* | [**delete_msg_vpn_bridge**](docs/MsgVpnApi.md#delete_msg_vpn_bridge) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Delete a Bridge object.
*MsgVpnApi* | [**delete_msg_vpn_bridge_remote_msg_vpn**](docs/MsgVpnApi.md#delete_msg_vpn_bridge_remote_msg_vpn) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Delete a Remote Message VPN object.
*MsgVpnApi* | [**delete_msg_vpn_bridge_remote_subscription**](docs/MsgVpnApi.md#delete_msg_vpn_bridge_remote_subscription) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Delete a Remote Subscription object.
*MsgVpnApi* | [**delete_msg_vpn_bridge_tls_trusted_common_name**](docs/MsgVpnApi.md#delete_msg_vpn_bridge_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*MsgVpnApi* | [**delete_msg_vpn_client_profile**](docs/MsgVpnApi.md#delete_msg_vpn_client_profile) | **Delete** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Delete a Client Profile object.
*MsgVpnApi* | [**delete_msg_vpn_client_username**](docs/MsgVpnApi.md#delete_msg_vpn_client_username) | **Delete** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Delete a Client Username object.
*MsgVpnApi* | [**delete_msg_vpn_distributed_cache**](docs/MsgVpnApi.md#delete_msg_vpn_distributed_cache) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Delete a Distributed Cache object.
*MsgVpnApi* | [**delete_msg_vpn_distributed_cache_cluster**](docs/MsgVpnApi.md#delete_msg_vpn_distributed_cache_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Delete a Cache Cluster object.
*MsgVpnApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/MsgVpnApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Delete a Home Cache Cluster object.
*MsgVpnApi* | [**delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/MsgVpnApi.md#delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Delete a Topic Prefix object.
*MsgVpnApi* | [**delete_msg_vpn_distributed_cache_cluster_instance**](docs/MsgVpnApi.md#delete_msg_vpn_distributed_cache_cluster_instance) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Delete a Cache Instance object.
*MsgVpnApi* | [**delete_msg_vpn_distributed_cache_cluster_topic**](docs/MsgVpnApi.md#delete_msg_vpn_distributed_cache_cluster_topic) | **Delete** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Delete a Topic object.
*MsgVpnApi* | [**delete_msg_vpn_dmr_bridge**](docs/MsgVpnApi.md#delete_msg_vpn_dmr_bridge) | **Delete** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Delete a DMR Bridge object.
*MsgVpnApi* | [**delete_msg_vpn_jndi_connection_factory**](docs/MsgVpnApi.md#delete_msg_vpn_jndi_connection_factory) | **Delete** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Delete a JNDI Connection Factory object.
*MsgVpnApi* | [**delete_msg_vpn_jndi_queue**](docs/MsgVpnApi.md#delete_msg_vpn_jndi_queue) | **Delete** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Delete a JNDI Queue object.
*MsgVpnApi* | [**delete_msg_vpn_jndi_topic**](docs/MsgVpnApi.md#delete_msg_vpn_jndi_topic) | **Delete** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Delete a JNDI Topic object.
*MsgVpnApi* | [**delete_msg_vpn_mqtt_retain_cache**](docs/MsgVpnApi.md#delete_msg_vpn_mqtt_retain_cache) | **Delete** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Delete an MQTT Retain Cache object.
*MsgVpnApi* | [**delete_msg_vpn_mqtt_session**](docs/MsgVpnApi.md#delete_msg_vpn_mqtt_session) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Delete an MQTT Session object.
*MsgVpnApi* | [**delete_msg_vpn_mqtt_session_subscription**](docs/MsgVpnApi.md#delete_msg_vpn_mqtt_session_subscription) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Delete a Subscription object.
*MsgVpnApi* | [**delete_msg_vpn_queue**](docs/MsgVpnApi.md#delete_msg_vpn_queue) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName} | Delete a Queue object.
*MsgVpnApi* | [**delete_msg_vpn_queue_subscription**](docs/MsgVpnApi.md#delete_msg_vpn_queue_subscription) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Delete a Queue Subscription object.
*MsgVpnApi* | [**delete_msg_vpn_queue_template**](docs/MsgVpnApi.md#delete_msg_vpn_queue_template) | **Delete** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Delete a Queue Template object.
*MsgVpnApi* | [**delete_msg_vpn_replay_log**](docs/MsgVpnApi.md#delete_msg_vpn_replay_log) | **Delete** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Delete a Replay Log object.
*MsgVpnApi* | [**delete_msg_vpn_replicated_topic**](docs/MsgVpnApi.md#delete_msg_vpn_replicated_topic) | **Delete** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Delete a Replicated Topic object.
*MsgVpnApi* | [**delete_msg_vpn_rest_delivery_point**](docs/MsgVpnApi.md#delete_msg_vpn_rest_delivery_point) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Delete a REST Delivery Point object.
*MsgVpnApi* | [**delete_msg_vpn_rest_delivery_point_queue_binding**](docs/MsgVpnApi.md#delete_msg_vpn_rest_delivery_point_queue_binding) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Delete a Queue Binding object.
*MsgVpnApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer**](docs/MsgVpnApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Delete a REST Consumer object.
*MsgVpnApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/MsgVpnApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*MsgVpnApi* | [**delete_msg_vpn_sequenced_topic**](docs/MsgVpnApi.md#delete_msg_vpn_sequenced_topic) | **Delete** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Delete a Sequenced Topic object.
*MsgVpnApi* | [**delete_msg_vpn_topic_endpoint**](docs/MsgVpnApi.md#delete_msg_vpn_topic_endpoint) | **Delete** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Delete a Topic Endpoint object.
*MsgVpnApi* | [**delete_msg_vpn_topic_endpoint_template**](docs/MsgVpnApi.md#delete_msg_vpn_topic_endpoint_template) | **Delete** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Delete a Topic Endpoint Template object.
*MsgVpnApi* | [**get_msg_vpn**](docs/MsgVpnApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Get a Message VPN object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Get an ACL Profile object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_client_connect_exception**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_client_connect_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Get a Client Connect Exception object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_client_connect_exceptions**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_client_connect_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Get a list of Client Connect Exception objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_publish_exception**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_publish_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Get a Publish Topic Exception object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_publish_exceptions**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_publish_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Get a list of Publish Topic Exception objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_publish_topic_exception**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_publish_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions/{publishTopicExceptionSyntax},{publishTopicException} | Get a Publish Topic Exception object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_publish_topic_exceptions**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_publish_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishTopicExceptions | Get a list of Publish Topic Exception objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_subscribe_exception**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Get a Subscribe Topic Exception object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_subscribe_exceptions**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Get a list of Subscribe Topic Exception objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_subscribe_share_name_exception**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions/{subscribeShareNameExceptionSyntax},{subscribeShareNameException} | Get a Subscribe Share Name Exception object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_subscribe_share_name_exceptions**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_subscribe_share_name_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeShareNameExceptions | Get a list of Subscribe Share Name Exception objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_subscribe_topic_exception**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_subscribe_topic_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions/{subscribeTopicExceptionSyntax},{subscribeTopicException} | Get a Subscribe Topic Exception object.
*MsgVpnApi* | [**get_msg_vpn_acl_profile_subscribe_topic_exceptions**](docs/MsgVpnApi.md#get_msg_vpn_acl_profile_subscribe_topic_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeTopicExceptions | Get a list of Subscribe Topic Exception objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profiles**](docs/MsgVpnApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Get a list of ACL Profile objects.
*MsgVpnApi* | [**get_msg_vpn_authentication_oauth_provider**](docs/MsgVpnApi.md#get_msg_vpn_authentication_oauth_provider) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Get an OAuth Provider object.
*MsgVpnApi* | [**get_msg_vpn_authentication_oauth_providers**](docs/MsgVpnApi.md#get_msg_vpn_authentication_oauth_providers) | **Get** /msgVpns/{msgVpnName}/authenticationOauthProviders | Get a list of OAuth Provider objects.
*MsgVpnApi* | [**get_msg_vpn_authorization_group**](docs/MsgVpnApi.md#get_msg_vpn_authorization_group) | **Get** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Get an LDAP Authorization Group object.
*MsgVpnApi* | [**get_msg_vpn_authorization_groups**](docs/MsgVpnApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Get a list of LDAP Authorization Group objects.
*MsgVpnApi* | [**get_msg_vpn_bridge**](docs/MsgVpnApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Get a Bridge object.
*MsgVpnApi* | [**get_msg_vpn_bridge_remote_msg_vpn**](docs/MsgVpnApi.md#get_msg_vpn_bridge_remote_msg_vpn) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Get a Remote Message VPN object.
*MsgVpnApi* | [**get_msg_vpn_bridge_remote_msg_vpns**](docs/MsgVpnApi.md#get_msg_vpn_bridge_remote_msg_vpns) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Get a list of Remote Message VPN objects.
*MsgVpnApi* | [**get_msg_vpn_bridge_remote_subscription**](docs/MsgVpnApi.md#get_msg_vpn_bridge_remote_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Get a Remote Subscription object.
*MsgVpnApi* | [**get_msg_vpn_bridge_remote_subscriptions**](docs/MsgVpnApi.md#get_msg_vpn_bridge_remote_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Get a list of Remote Subscription objects.
*MsgVpnApi* | [**get_msg_vpn_bridge_tls_trusted_common_name**](docs/MsgVpnApi.md#get_msg_vpn_bridge_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*MsgVpnApi* | [**get_msg_vpn_bridge_tls_trusted_common_names**](docs/MsgVpnApi.md#get_msg_vpn_bridge_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*MsgVpnApi* | [**get_msg_vpn_bridges**](docs/MsgVpnApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Get a list of Bridge objects.
*MsgVpnApi* | [**get_msg_vpn_client_profile**](docs/MsgVpnApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Get a Client Profile object.
*MsgVpnApi* | [**get_msg_vpn_client_profiles**](docs/MsgVpnApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Get a list of Client Profile objects.
*MsgVpnApi* | [**get_msg_vpn_client_username**](docs/MsgVpnApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Get a Client Username object.
*MsgVpnApi* | [**get_msg_vpn_client_usernames**](docs/MsgVpnApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Get a list of Client Username objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Get a Distributed Cache object.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Get a Cache Cluster object.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName} | Get a Home Cache Cluster object.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix} | Get a Topic Prefix object.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes | Get a list of Topic Prefix objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters | Get a list of Home Cache Cluster objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_instance**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_instance) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Get a Cache Instance object.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_instances**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_instances) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances | Get a list of Cache Instance objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_topic**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_topic) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic} | Get a Topic object.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_cluster_topics**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_cluster_topics) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics | Get a list of Topic objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_cache_clusters**](docs/MsgVpnApi.md#get_msg_vpn_distributed_cache_clusters) | **Get** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters | Get a list of Cache Cluster objects.
*MsgVpnApi* | [**get_msg_vpn_distributed_caches**](docs/MsgVpnApi.md#get_msg_vpn_distributed_caches) | **Get** /msgVpns/{msgVpnName}/distributedCaches | Get a list of Distributed Cache objects.
*MsgVpnApi* | [**get_msg_vpn_dmr_bridge**](docs/MsgVpnApi.md#get_msg_vpn_dmr_bridge) | **Get** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Get a DMR Bridge object.
*MsgVpnApi* | [**get_msg_vpn_dmr_bridges**](docs/MsgVpnApi.md#get_msg_vpn_dmr_bridges) | **Get** /msgVpns/{msgVpnName}/dmrBridges | Get a list of DMR Bridge objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_connection_factories**](docs/MsgVpnApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Get a list of JNDI Connection Factory objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_connection_factory**](docs/MsgVpnApi.md#get_msg_vpn_jndi_connection_factory) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Get a JNDI Connection Factory object.
*MsgVpnApi* | [**get_msg_vpn_jndi_queue**](docs/MsgVpnApi.md#get_msg_vpn_jndi_queue) | **Get** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Get a JNDI Queue object.
*MsgVpnApi* | [**get_msg_vpn_jndi_queues**](docs/MsgVpnApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Get a list of JNDI Queue objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_topic**](docs/MsgVpnApi.md#get_msg_vpn_jndi_topic) | **Get** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Get a JNDI Topic object.
*MsgVpnApi* | [**get_msg_vpn_jndi_topics**](docs/MsgVpnApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Get a list of JNDI Topic objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_retain_cache**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_retain_cache) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Get an MQTT Retain Cache object.
*MsgVpnApi* | [**get_msg_vpn_mqtt_retain_caches**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_retain_caches) | **Get** /msgVpns/{msgVpnName}/mqttRetainCaches | Get a list of MQTT Retain Cache objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_session**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Get an MQTT Session object.
*MsgVpnApi* | [**get_msg_vpn_mqtt_session_subscription**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_session_subscription) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Get a Subscription object.
*MsgVpnApi* | [**get_msg_vpn_mqtt_session_subscriptions**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_session_subscriptions) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Get a list of Subscription objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_sessions**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Get a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_queue**](docs/MsgVpnApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
*MsgVpnApi* | [**get_msg_vpn_queue_subscription**](docs/MsgVpnApi.md#get_msg_vpn_queue_subscription) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Get a Queue Subscription object.
*MsgVpnApi* | [**get_msg_vpn_queue_subscriptions**](docs/MsgVpnApi.md#get_msg_vpn_queue_subscriptions) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Get a list of Queue Subscription objects.
*MsgVpnApi* | [**get_msg_vpn_queue_template**](docs/MsgVpnApi.md#get_msg_vpn_queue_template) | **Get** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Get a Queue Template object.
*MsgVpnApi* | [**get_msg_vpn_queue_templates**](docs/MsgVpnApi.md#get_msg_vpn_queue_templates) | **Get** /msgVpns/{msgVpnName}/queueTemplates | Get a list of Queue Template objects.
*MsgVpnApi* | [**get_msg_vpn_queues**](docs/MsgVpnApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*MsgVpnApi* | [**get_msg_vpn_replay_log**](docs/MsgVpnApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
*MsgVpnApi* | [**get_msg_vpn_replay_logs**](docs/MsgVpnApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*MsgVpnApi* | [**get_msg_vpn_replicated_topic**](docs/MsgVpnApi.md#get_msg_vpn_replicated_topic) | **Get** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Get a Replicated Topic object.
*MsgVpnApi* | [**get_msg_vpn_replicated_topics**](docs/MsgVpnApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Get a list of Replicated Topic objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point_queue_binding**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Get a Queue Binding object.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point_queue_bindings**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Get a list of Queue Binding objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_point_rest_consumers**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_points**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*MsgVpnApi* | [**get_msg_vpn_sequenced_topic**](docs/MsgVpnApi.md#get_msg_vpn_sequenced_topic) | **Get** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Get a Sequenced Topic object.
*MsgVpnApi* | [**get_msg_vpn_sequenced_topics**](docs/MsgVpnApi.md#get_msg_vpn_sequenced_topics) | **Get** /msgVpns/{msgVpnName}/sequencedTopics | Get a list of Sequenced Topic objects.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoint**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoint_template**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoint_template) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Get a Topic Endpoint Template object.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoint_templates**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoint_templates) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates | Get a list of Topic Endpoint Template objects.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoints**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.
*MsgVpnApi* | [**get_msg_vpns**](docs/MsgVpnApi.md#get_msg_vpns) | **Get** /msgVpns | Get a list of Message VPN objects.
*MsgVpnApi* | [**replace_msg_vpn**](docs/MsgVpnApi.md#replace_msg_vpn) | **Put** /msgVpns/{msgVpnName} | Replace a Message VPN object.
*MsgVpnApi* | [**replace_msg_vpn_acl_profile**](docs/MsgVpnApi.md#replace_msg_vpn_acl_profile) | **Put** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Replace an ACL Profile object.
*MsgVpnApi* | [**replace_msg_vpn_authentication_oauth_provider**](docs/MsgVpnApi.md#replace_msg_vpn_authentication_oauth_provider) | **Put** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Replace an OAuth Provider object.
*MsgVpnApi* | [**replace_msg_vpn_authorization_group**](docs/MsgVpnApi.md#replace_msg_vpn_authorization_group) | **Put** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Replace an LDAP Authorization Group object.
*MsgVpnApi* | [**replace_msg_vpn_bridge**](docs/MsgVpnApi.md#replace_msg_vpn_bridge) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Replace a Bridge object.
*MsgVpnApi* | [**replace_msg_vpn_bridge_remote_msg_vpn**](docs/MsgVpnApi.md#replace_msg_vpn_bridge_remote_msg_vpn) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Replace a Remote Message VPN object.
*MsgVpnApi* | [**replace_msg_vpn_client_profile**](docs/MsgVpnApi.md#replace_msg_vpn_client_profile) | **Put** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Replace a Client Profile object.
*MsgVpnApi* | [**replace_msg_vpn_client_username**](docs/MsgVpnApi.md#replace_msg_vpn_client_username) | **Put** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Replace a Client Username object.
*MsgVpnApi* | [**replace_msg_vpn_distributed_cache**](docs/MsgVpnApi.md#replace_msg_vpn_distributed_cache) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Replace a Distributed Cache object.
*MsgVpnApi* | [**replace_msg_vpn_distributed_cache_cluster**](docs/MsgVpnApi.md#replace_msg_vpn_distributed_cache_cluster) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Replace a Cache Cluster object.
*MsgVpnApi* | [**replace_msg_vpn_distributed_cache_cluster_instance**](docs/MsgVpnApi.md#replace_msg_vpn_distributed_cache_cluster_instance) | **Put** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Replace a Cache Instance object.
*MsgVpnApi* | [**replace_msg_vpn_dmr_bridge**](docs/MsgVpnApi.md#replace_msg_vpn_dmr_bridge) | **Put** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Replace a DMR Bridge object.
*MsgVpnApi* | [**replace_msg_vpn_jndi_connection_factory**](docs/MsgVpnApi.md#replace_msg_vpn_jndi_connection_factory) | **Put** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Replace a JNDI Connection Factory object.
*MsgVpnApi* | [**replace_msg_vpn_jndi_queue**](docs/MsgVpnApi.md#replace_msg_vpn_jndi_queue) | **Put** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Replace a JNDI Queue object.
*MsgVpnApi* | [**replace_msg_vpn_jndi_topic**](docs/MsgVpnApi.md#replace_msg_vpn_jndi_topic) | **Put** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Replace a JNDI Topic object.
*MsgVpnApi* | [**replace_msg_vpn_mqtt_retain_cache**](docs/MsgVpnApi.md#replace_msg_vpn_mqtt_retain_cache) | **Put** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Replace an MQTT Retain Cache object.
*MsgVpnApi* | [**replace_msg_vpn_mqtt_session**](docs/MsgVpnApi.md#replace_msg_vpn_mqtt_session) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Replace an MQTT Session object.
*MsgVpnApi* | [**replace_msg_vpn_mqtt_session_subscription**](docs/MsgVpnApi.md#replace_msg_vpn_mqtt_session_subscription) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Replace a Subscription object.
*MsgVpnApi* | [**replace_msg_vpn_queue**](docs/MsgVpnApi.md#replace_msg_vpn_queue) | **Put** /msgVpns/{msgVpnName}/queues/{queueName} | Replace a Queue object.
*MsgVpnApi* | [**replace_msg_vpn_queue_template**](docs/MsgVpnApi.md#replace_msg_vpn_queue_template) | **Put** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Replace a Queue Template object.
*MsgVpnApi* | [**replace_msg_vpn_replay_log**](docs/MsgVpnApi.md#replace_msg_vpn_replay_log) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Replace a Replay Log object.
*MsgVpnApi* | [**replace_msg_vpn_replicated_topic**](docs/MsgVpnApi.md#replace_msg_vpn_replicated_topic) | **Put** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Replace a Replicated Topic object.
*MsgVpnApi* | [**replace_msg_vpn_rest_delivery_point**](docs/MsgVpnApi.md#replace_msg_vpn_rest_delivery_point) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Replace a REST Delivery Point object.
*MsgVpnApi* | [**replace_msg_vpn_rest_delivery_point_queue_binding**](docs/MsgVpnApi.md#replace_msg_vpn_rest_delivery_point_queue_binding) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Replace a Queue Binding object.
*MsgVpnApi* | [**replace_msg_vpn_rest_delivery_point_rest_consumer**](docs/MsgVpnApi.md#replace_msg_vpn_rest_delivery_point_rest_consumer) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Replace a REST Consumer object.
*MsgVpnApi* | [**replace_msg_vpn_topic_endpoint**](docs/MsgVpnApi.md#replace_msg_vpn_topic_endpoint) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Replace a Topic Endpoint object.
*MsgVpnApi* | [**replace_msg_vpn_topic_endpoint_template**](docs/MsgVpnApi.md#replace_msg_vpn_topic_endpoint_template) | **Put** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Replace a Topic Endpoint Template object.
*MsgVpnApi* | [**update_msg_vpn**](docs/MsgVpnApi.md#update_msg_vpn) | **Patch** /msgVpns/{msgVpnName} | Update a Message VPN object.
*MsgVpnApi* | [**update_msg_vpn_acl_profile**](docs/MsgVpnApi.md#update_msg_vpn_acl_profile) | **Patch** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Update an ACL Profile object.
*MsgVpnApi* | [**update_msg_vpn_authentication_oauth_provider**](docs/MsgVpnApi.md#update_msg_vpn_authentication_oauth_provider) | **Patch** /msgVpns/{msgVpnName}/authenticationOauthProviders/{oauthProviderName} | Update an OAuth Provider object.
*MsgVpnApi* | [**update_msg_vpn_authorization_group**](docs/MsgVpnApi.md#update_msg_vpn_authorization_group) | **Patch** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Update an LDAP Authorization Group object.
*MsgVpnApi* | [**update_msg_vpn_bridge**](docs/MsgVpnApi.md#update_msg_vpn_bridge) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Update a Bridge object.
*MsgVpnApi* | [**update_msg_vpn_bridge_remote_msg_vpn**](docs/MsgVpnApi.md#update_msg_vpn_bridge_remote_msg_vpn) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Update a Remote Message VPN object.
*MsgVpnApi* | [**update_msg_vpn_client_profile**](docs/MsgVpnApi.md#update_msg_vpn_client_profile) | **Patch** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Update a Client Profile object.
*MsgVpnApi* | [**update_msg_vpn_client_username**](docs/MsgVpnApi.md#update_msg_vpn_client_username) | **Patch** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Update a Client Username object.
*MsgVpnApi* | [**update_msg_vpn_distributed_cache**](docs/MsgVpnApi.md#update_msg_vpn_distributed_cache) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName} | Update a Distributed Cache object.
*MsgVpnApi* | [**update_msg_vpn_distributed_cache_cluster**](docs/MsgVpnApi.md#update_msg_vpn_distributed_cache_cluster) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName} | Update a Cache Cluster object.
*MsgVpnApi* | [**update_msg_vpn_distributed_cache_cluster_instance**](docs/MsgVpnApi.md#update_msg_vpn_distributed_cache_cluster_instance) | **Patch** /msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName} | Update a Cache Instance object.
*MsgVpnApi* | [**update_msg_vpn_dmr_bridge**](docs/MsgVpnApi.md#update_msg_vpn_dmr_bridge) | **Patch** /msgVpns/{msgVpnName}/dmrBridges/{remoteNodeName} | Update a DMR Bridge object.
*MsgVpnApi* | [**update_msg_vpn_jndi_connection_factory**](docs/MsgVpnApi.md#update_msg_vpn_jndi_connection_factory) | **Patch** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Update a JNDI Connection Factory object.
*MsgVpnApi* | [**update_msg_vpn_jndi_queue**](docs/MsgVpnApi.md#update_msg_vpn_jndi_queue) | **Patch** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Update a JNDI Queue object.
*MsgVpnApi* | [**update_msg_vpn_jndi_topic**](docs/MsgVpnApi.md#update_msg_vpn_jndi_topic) | **Patch** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Update a JNDI Topic object.
*MsgVpnApi* | [**update_msg_vpn_mqtt_retain_cache**](docs/MsgVpnApi.md#update_msg_vpn_mqtt_retain_cache) | **Patch** /msgVpns/{msgVpnName}/mqttRetainCaches/{cacheName} | Update an MQTT Retain Cache object.
*MsgVpnApi* | [**update_msg_vpn_mqtt_session**](docs/MsgVpnApi.md#update_msg_vpn_mqtt_session) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Update an MQTT Session object.
*MsgVpnApi* | [**update_msg_vpn_mqtt_session_subscription**](docs/MsgVpnApi.md#update_msg_vpn_mqtt_session_subscription) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Update a Subscription object.
*MsgVpnApi* | [**update_msg_vpn_queue**](docs/MsgVpnApi.md#update_msg_vpn_queue) | **Patch** /msgVpns/{msgVpnName}/queues/{queueName} | Update a Queue object.
*MsgVpnApi* | [**update_msg_vpn_queue_template**](docs/MsgVpnApi.md#update_msg_vpn_queue_template) | **Patch** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Update a Queue Template object.
*MsgVpnApi* | [**update_msg_vpn_replay_log**](docs/MsgVpnApi.md#update_msg_vpn_replay_log) | **Patch** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Update a Replay Log object.
*MsgVpnApi* | [**update_msg_vpn_replicated_topic**](docs/MsgVpnApi.md#update_msg_vpn_replicated_topic) | **Patch** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Update a Replicated Topic object.
*MsgVpnApi* | [**update_msg_vpn_rest_delivery_point**](docs/MsgVpnApi.md#update_msg_vpn_rest_delivery_point) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Update a REST Delivery Point object.
*MsgVpnApi* | [**update_msg_vpn_rest_delivery_point_queue_binding**](docs/MsgVpnApi.md#update_msg_vpn_rest_delivery_point_queue_binding) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Update a Queue Binding object.
*MsgVpnApi* | [**update_msg_vpn_rest_delivery_point_rest_consumer**](docs/MsgVpnApi.md#update_msg_vpn_rest_delivery_point_rest_consumer) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Update a REST Consumer object.
*MsgVpnApi* | [**update_msg_vpn_topic_endpoint**](docs/MsgVpnApi.md#update_msg_vpn_topic_endpoint) | **Patch** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Update a Topic Endpoint object.
*MsgVpnApi* | [**update_msg_vpn_topic_endpoint_template**](docs/MsgVpnApi.md#update_msg_vpn_topic_endpoint_template) | **Patch** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Update a Topic Endpoint Template object.
*QueueApi* | [**create_msg_vpn_queue**](docs/QueueApi.md#create_msg_vpn_queue) | **Post** /msgVpns/{msgVpnName}/queues | Create a Queue object.
*QueueApi* | [**create_msg_vpn_queue_subscription**](docs/QueueApi.md#create_msg_vpn_queue_subscription) | **Post** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Create a Queue Subscription object.
*QueueApi* | [**delete_msg_vpn_queue**](docs/QueueApi.md#delete_msg_vpn_queue) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName} | Delete a Queue object.
*QueueApi* | [**delete_msg_vpn_queue_subscription**](docs/QueueApi.md#delete_msg_vpn_queue_subscription) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Delete a Queue Subscription object.
*QueueApi* | [**get_msg_vpn_queue**](docs/QueueApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Get a Queue object.
*QueueApi* | [**get_msg_vpn_queue_subscription**](docs/QueueApi.md#get_msg_vpn_queue_subscription) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Get a Queue Subscription object.
*QueueApi* | [**get_msg_vpn_queue_subscriptions**](docs/QueueApi.md#get_msg_vpn_queue_subscriptions) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Get a list of Queue Subscription objects.
*QueueApi* | [**get_msg_vpn_queues**](docs/QueueApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Get a list of Queue objects.
*QueueApi* | [**replace_msg_vpn_queue**](docs/QueueApi.md#replace_msg_vpn_queue) | **Put** /msgVpns/{msgVpnName}/queues/{queueName} | Replace a Queue object.
*QueueApi* | [**update_msg_vpn_queue**](docs/QueueApi.md#update_msg_vpn_queue) | **Patch** /msgVpns/{msgVpnName}/queues/{queueName} | Update a Queue object.
*QueueTemplateApi* | [**create_msg_vpn_queue_template**](docs/QueueTemplateApi.md#create_msg_vpn_queue_template) | **Post** /msgVpns/{msgVpnName}/queueTemplates | Create a Queue Template object.
*QueueTemplateApi* | [**delete_msg_vpn_queue_template**](docs/QueueTemplateApi.md#delete_msg_vpn_queue_template) | **Delete** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Delete a Queue Template object.
*QueueTemplateApi* | [**get_msg_vpn_queue_template**](docs/QueueTemplateApi.md#get_msg_vpn_queue_template) | **Get** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Get a Queue Template object.
*QueueTemplateApi* | [**get_msg_vpn_queue_templates**](docs/QueueTemplateApi.md#get_msg_vpn_queue_templates) | **Get** /msgVpns/{msgVpnName}/queueTemplates | Get a list of Queue Template objects.
*QueueTemplateApi* | [**replace_msg_vpn_queue_template**](docs/QueueTemplateApi.md#replace_msg_vpn_queue_template) | **Put** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Replace a Queue Template object.
*QueueTemplateApi* | [**update_msg_vpn_queue_template**](docs/QueueTemplateApi.md#update_msg_vpn_queue_template) | **Patch** /msgVpns/{msgVpnName}/queueTemplates/{queueTemplateName} | Update a Queue Template object.
*ReplayLogApi* | [**create_msg_vpn_replay_log**](docs/ReplayLogApi.md#create_msg_vpn_replay_log) | **Post** /msgVpns/{msgVpnName}/replayLogs | Create a Replay Log object.
*ReplayLogApi* | [**delete_msg_vpn_replay_log**](docs/ReplayLogApi.md#delete_msg_vpn_replay_log) | **Delete** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Delete a Replay Log object.
*ReplayLogApi* | [**get_msg_vpn_replay_log**](docs/ReplayLogApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Get a Replay Log object.
*ReplayLogApi* | [**get_msg_vpn_replay_logs**](docs/ReplayLogApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Get a list of Replay Log objects.
*ReplayLogApi* | [**replace_msg_vpn_replay_log**](docs/ReplayLogApi.md#replace_msg_vpn_replay_log) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Replace a Replay Log object.
*ReplayLogApi* | [**update_msg_vpn_replay_log**](docs/ReplayLogApi.md#update_msg_vpn_replay_log) | **Patch** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Update a Replay Log object.
*ReplicatedTopicApi* | [**create_msg_vpn_replicated_topic**](docs/ReplicatedTopicApi.md#create_msg_vpn_replicated_topic) | **Post** /msgVpns/{msgVpnName}/replicatedTopics | Create a Replicated Topic object.
*ReplicatedTopicApi* | [**delete_msg_vpn_replicated_topic**](docs/ReplicatedTopicApi.md#delete_msg_vpn_replicated_topic) | **Delete** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Delete a Replicated Topic object.
*ReplicatedTopicApi* | [**get_msg_vpn_replicated_topic**](docs/ReplicatedTopicApi.md#get_msg_vpn_replicated_topic) | **Get** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Get a Replicated Topic object.
*ReplicatedTopicApi* | [**get_msg_vpn_replicated_topics**](docs/ReplicatedTopicApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Get a list of Replicated Topic objects.
*ReplicatedTopicApi* | [**replace_msg_vpn_replicated_topic**](docs/ReplicatedTopicApi.md#replace_msg_vpn_replicated_topic) | **Put** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Replace a Replicated Topic object.
*ReplicatedTopicApi* | [**update_msg_vpn_replicated_topic**](docs/ReplicatedTopicApi.md#update_msg_vpn_replicated_topic) | **Patch** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Update a Replicated Topic object.
*RestDeliveryPointApi* | [**create_msg_vpn_rest_delivery_point**](docs/RestDeliveryPointApi.md#create_msg_vpn_rest_delivery_point) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints | Create a REST Delivery Point object.
*RestDeliveryPointApi* | [**create_msg_vpn_rest_delivery_point_queue_binding**](docs/RestDeliveryPointApi.md#create_msg_vpn_rest_delivery_point_queue_binding) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Create a Queue Binding object.
*RestDeliveryPointApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer**](docs/RestDeliveryPointApi.md#create_msg_vpn_rest_delivery_point_rest_consumer) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Create a REST Consumer object.
*RestDeliveryPointApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/RestDeliveryPointApi.md#create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Create a Trusted Common Name object.
*RestDeliveryPointApi* | [**delete_msg_vpn_rest_delivery_point**](docs/RestDeliveryPointApi.md#delete_msg_vpn_rest_delivery_point) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Delete a REST Delivery Point object.
*RestDeliveryPointApi* | [**delete_msg_vpn_rest_delivery_point_queue_binding**](docs/RestDeliveryPointApi.md#delete_msg_vpn_rest_delivery_point_queue_binding) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Delete a Queue Binding object.
*RestDeliveryPointApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer**](docs/RestDeliveryPointApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Delete a REST Consumer object.
*RestDeliveryPointApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/RestDeliveryPointApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Delete a Trusted Common Name object.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Get a REST Delivery Point object.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point_queue_binding**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Get a Queue Binding object.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point_queue_bindings**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Get a list of Queue Binding objects.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Get a REST Consumer object.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Get a Trusted Common Name object.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Get a list of Trusted Common Name objects.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_point_rest_consumers**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Get a list of REST Consumer objects.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_points**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Get a list of REST Delivery Point objects.
*RestDeliveryPointApi* | [**replace_msg_vpn_rest_delivery_point**](docs/RestDeliveryPointApi.md#replace_msg_vpn_rest_delivery_point) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Replace a REST Delivery Point object.
*RestDeliveryPointApi* | [**replace_msg_vpn_rest_delivery_point_queue_binding**](docs/RestDeliveryPointApi.md#replace_msg_vpn_rest_delivery_point_queue_binding) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Replace a Queue Binding object.
*RestDeliveryPointApi* | [**replace_msg_vpn_rest_delivery_point_rest_consumer**](docs/RestDeliveryPointApi.md#replace_msg_vpn_rest_delivery_point_rest_consumer) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Replace a REST Consumer object.
*RestDeliveryPointApi* | [**update_msg_vpn_rest_delivery_point**](docs/RestDeliveryPointApi.md#update_msg_vpn_rest_delivery_point) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Update a REST Delivery Point object.
*RestDeliveryPointApi* | [**update_msg_vpn_rest_delivery_point_queue_binding**](docs/RestDeliveryPointApi.md#update_msg_vpn_rest_delivery_point_queue_binding) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Update a Queue Binding object.
*RestDeliveryPointApi* | [**update_msg_vpn_rest_delivery_point_rest_consumer**](docs/RestDeliveryPointApi.md#update_msg_vpn_rest_delivery_point_rest_consumer) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Update a REST Consumer object.
*SystemInformationApi* | [**get_system_information**](docs/SystemInformationApi.md#get_system_information) | **Get** /systemInformation | Get a System Information object.
*TopicEndpointApi* | [**create_msg_vpn_topic_endpoint**](docs/TopicEndpointApi.md#create_msg_vpn_topic_endpoint) | **Post** /msgVpns/{msgVpnName}/topicEndpoints | Create a Topic Endpoint object.
*TopicEndpointApi* | [**delete_msg_vpn_topic_endpoint**](docs/TopicEndpointApi.md#delete_msg_vpn_topic_endpoint) | **Delete** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Delete a Topic Endpoint object.
*TopicEndpointApi* | [**get_msg_vpn_topic_endpoint**](docs/TopicEndpointApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Get a Topic Endpoint object.
*TopicEndpointApi* | [**get_msg_vpn_topic_endpoints**](docs/TopicEndpointApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Get a list of Topic Endpoint objects.
*TopicEndpointApi* | [**replace_msg_vpn_topic_endpoint**](docs/TopicEndpointApi.md#replace_msg_vpn_topic_endpoint) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Replace a Topic Endpoint object.
*TopicEndpointApi* | [**update_msg_vpn_topic_endpoint**](docs/TopicEndpointApi.md#update_msg_vpn_topic_endpoint) | **Patch** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Update a Topic Endpoint object.
*TopicEndpointTemplateApi* | [**create_msg_vpn_topic_endpoint_template**](docs/TopicEndpointTemplateApi.md#create_msg_vpn_topic_endpoint_template) | **Post** /msgVpns/{msgVpnName}/topicEndpointTemplates | Create a Topic Endpoint Template object.
*TopicEndpointTemplateApi* | [**delete_msg_vpn_topic_endpoint_template**](docs/TopicEndpointTemplateApi.md#delete_msg_vpn_topic_endpoint_template) | **Delete** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Delete a Topic Endpoint Template object.
*TopicEndpointTemplateApi* | [**get_msg_vpn_topic_endpoint_template**](docs/TopicEndpointTemplateApi.md#get_msg_vpn_topic_endpoint_template) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Get a Topic Endpoint Template object.
*TopicEndpointTemplateApi* | [**get_msg_vpn_topic_endpoint_templates**](docs/TopicEndpointTemplateApi.md#get_msg_vpn_topic_endpoint_templates) | **Get** /msgVpns/{msgVpnName}/topicEndpointTemplates | Get a list of Topic Endpoint Template objects.
*TopicEndpointTemplateApi* | [**replace_msg_vpn_topic_endpoint_template**](docs/TopicEndpointTemplateApi.md#replace_msg_vpn_topic_endpoint_template) | **Put** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Replace a Topic Endpoint Template object.
*TopicEndpointTemplateApi* | [**update_msg_vpn_topic_endpoint_template**](docs/TopicEndpointTemplateApi.md#update_msg_vpn_topic_endpoint_template) | **Patch** /msgVpns/{msgVpnName}/topicEndpointTemplates/{topicEndpointTemplateName} | Update a Topic Endpoint Template object.
*VirtualHostnameApi* | [**create_virtual_hostname**](docs/VirtualHostnameApi.md#create_virtual_hostname) | **Post** /virtualHostnames | Create a Virtual Hostname object.
*VirtualHostnameApi* | [**delete_virtual_hostname**](docs/VirtualHostnameApi.md#delete_virtual_hostname) | **Delete** /virtualHostnames/{virtualHostname} | Delete a Virtual Hostname object.
*VirtualHostnameApi* | [**get_virtual_hostname**](docs/VirtualHostnameApi.md#get_virtual_hostname) | **Get** /virtualHostnames/{virtualHostname} | Get a Virtual Hostname object.
*VirtualHostnameApi* | [**get_virtual_hostnames**](docs/VirtualHostnameApi.md#get_virtual_hostnames) | **Get** /virtualHostnames | Get a list of Virtual Hostname objects.
*VirtualHostnameApi* | [**replace_virtual_hostname**](docs/VirtualHostnameApi.md#replace_virtual_hostname) | **Put** /virtualHostnames/{virtualHostname} | Replace a Virtual Hostname object.
*VirtualHostnameApi* | [**update_virtual_hostname**](docs/VirtualHostnameApi.md#update_virtual_hostname) | **Patch** /virtualHostnames/{virtualHostname} | Update a Virtual Hostname object.


## Documentation For Models

 - [About](docs/About.md)
 - [AboutApi](docs/AboutApi.md)
 - [AboutApiLinks](docs/AboutApiLinks.md)
 - [AboutApiResponse](docs/AboutApiResponse.md)
 - [AboutLinks](docs/AboutLinks.md)
 - [AboutResponse](docs/AboutResponse.md)
 - [AboutUser](docs/AboutUser.md)
 - [AboutUserLinks](docs/AboutUserLinks.md)
 - [AboutUserMsgVpn](docs/AboutUserMsgVpn.md)
 - [AboutUserMsgVpnLinks](docs/AboutUserMsgVpnLinks.md)
 - [AboutUserMsgVpnResponse](docs/AboutUserMsgVpnResponse.md)
 - [AboutUserMsgVpnsResponse](docs/AboutUserMsgVpnsResponse.md)
 - [AboutUserResponse](docs/AboutUserResponse.md)
 - [Broker](docs/Broker.md)
 - [BrokerLinks](docs/BrokerLinks.md)
 - [BrokerResponse](docs/BrokerResponse.md)
 - [CertAuthoritiesResponse](docs/CertAuthoritiesResponse.md)
 - [CertAuthority](docs/CertAuthority.md)
 - [CertAuthorityLinks](docs/CertAuthorityLinks.md)
 - [CertAuthorityOcspTlsTrustedCommonName](docs/CertAuthorityOcspTlsTrustedCommonName.md)
 - [CertAuthorityOcspTlsTrustedCommonNameLinks](docs/CertAuthorityOcspTlsTrustedCommonNameLinks.md)
 - [CertAuthorityOcspTlsTrustedCommonNameResponse](docs/CertAuthorityOcspTlsTrustedCommonNameResponse.md)
 - [CertAuthorityOcspTlsTrustedCommonNamesResponse](docs/CertAuthorityOcspTlsTrustedCommonNamesResponse.md)
 - [CertAuthorityResponse](docs/CertAuthorityResponse.md)
 - [ClientCertAuthoritiesResponse](docs/ClientCertAuthoritiesResponse.md)
 - [ClientCertAuthority](docs/ClientCertAuthority.md)
 - [ClientCertAuthorityLinks](docs/ClientCertAuthorityLinks.md)
 - [ClientCertAuthorityOcspTlsTrustedCommonName](docs/ClientCertAuthorityOcspTlsTrustedCommonName.md)
 - [ClientCertAuthorityOcspTlsTrustedCommonNameLinks](docs/ClientCertAuthorityOcspTlsTrustedCommonNameLinks.md)
 - [ClientCertAuthorityOcspTlsTrustedCommonNameResponse](docs/ClientCertAuthorityOcspTlsTrustedCommonNameResponse.md)
 - [ClientCertAuthorityOcspTlsTrustedCommonNamesResponse](docs/ClientCertAuthorityOcspTlsTrustedCommonNamesResponse.md)
 - [ClientCertAuthorityResponse](docs/ClientCertAuthorityResponse.md)
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
 - [DomainCertAuthoritiesResponse](docs/DomainCertAuthoritiesResponse.md)
 - [DomainCertAuthority](docs/DomainCertAuthority.md)
 - [DomainCertAuthorityLinks](docs/DomainCertAuthorityLinks.md)
 - [DomainCertAuthorityResponse](docs/DomainCertAuthorityResponse.md)
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
 - [MsgVpnAclProfilePublishTopicException](docs/MsgVpnAclProfilePublishTopicException.md)
 - [MsgVpnAclProfilePublishTopicExceptionLinks](docs/MsgVpnAclProfilePublishTopicExceptionLinks.md)
 - [MsgVpnAclProfilePublishTopicExceptionResponse](docs/MsgVpnAclProfilePublishTopicExceptionResponse.md)
 - [MsgVpnAclProfilePublishTopicExceptionsResponse](docs/MsgVpnAclProfilePublishTopicExceptionsResponse.md)
 - [MsgVpnAclProfileResponse](docs/MsgVpnAclProfileResponse.md)
 - [MsgVpnAclProfileSubscribeException](docs/MsgVpnAclProfileSubscribeException.md)
 - [MsgVpnAclProfileSubscribeExceptionLinks](docs/MsgVpnAclProfileSubscribeExceptionLinks.md)
 - [MsgVpnAclProfileSubscribeExceptionResponse](docs/MsgVpnAclProfileSubscribeExceptionResponse.md)
 - [MsgVpnAclProfileSubscribeExceptionsResponse](docs/MsgVpnAclProfileSubscribeExceptionsResponse.md)
 - [MsgVpnAclProfileSubscribeShareNameException](docs/MsgVpnAclProfileSubscribeShareNameException.md)
 - [MsgVpnAclProfileSubscribeShareNameExceptionLinks](docs/MsgVpnAclProfileSubscribeShareNameExceptionLinks.md)
 - [MsgVpnAclProfileSubscribeShareNameExceptionResponse](docs/MsgVpnAclProfileSubscribeShareNameExceptionResponse.md)
 - [MsgVpnAclProfileSubscribeShareNameExceptionsResponse](docs/MsgVpnAclProfileSubscribeShareNameExceptionsResponse.md)
 - [MsgVpnAclProfileSubscribeTopicException](docs/MsgVpnAclProfileSubscribeTopicException.md)
 - [MsgVpnAclProfileSubscribeTopicExceptionLinks](docs/MsgVpnAclProfileSubscribeTopicExceptionLinks.md)
 - [MsgVpnAclProfileSubscribeTopicExceptionResponse](docs/MsgVpnAclProfileSubscribeTopicExceptionResponse.md)
 - [MsgVpnAclProfileSubscribeTopicExceptionsResponse](docs/MsgVpnAclProfileSubscribeTopicExceptionsResponse.md)
 - [MsgVpnAclProfilesResponse](docs/MsgVpnAclProfilesResponse.md)
 - [MsgVpnAuthenticationOauthProvider](docs/MsgVpnAuthenticationOauthProvider.md)
 - [MsgVpnAuthenticationOauthProviderLinks](docs/MsgVpnAuthenticationOauthProviderLinks.md)
 - [MsgVpnAuthenticationOauthProviderResponse](docs/MsgVpnAuthenticationOauthProviderResponse.md)
 - [MsgVpnAuthenticationOauthProvidersResponse](docs/MsgVpnAuthenticationOauthProvidersResponse.md)
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
 - [MsgVpnQueueTemplate](docs/MsgVpnQueueTemplate.md)
 - [MsgVpnQueueTemplateLinks](docs/MsgVpnQueueTemplateLinks.md)
 - [MsgVpnQueueTemplateResponse](docs/MsgVpnQueueTemplateResponse.md)
 - [MsgVpnQueueTemplatesResponse](docs/MsgVpnQueueTemplatesResponse.md)
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
 - [MsgVpnTopicEndpointTemplate](docs/MsgVpnTopicEndpointTemplate.md)
 - [MsgVpnTopicEndpointTemplateLinks](docs/MsgVpnTopicEndpointTemplateLinks.md)
 - [MsgVpnTopicEndpointTemplateResponse](docs/MsgVpnTopicEndpointTemplateResponse.md)
 - [MsgVpnTopicEndpointTemplatesResponse](docs/MsgVpnTopicEndpointTemplatesResponse.md)
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
 - [VirtualHostname](docs/VirtualHostname.md)
 - [VirtualHostnameLinks](docs/VirtualHostnameLinks.md)
 - [VirtualHostnameResponse](docs/VirtualHostnameResponse.md)
 - [VirtualHostnamesResponse](docs/VirtualHostnamesResponse.md)


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

