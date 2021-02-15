/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see note 1) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+ broker. Resources are individual **objects**, **collections** of objects, or (exclusively in the action API) **actions**. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See note 2    The following APIs are also available:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Action|/SEMP/v2/action|Performing actions|See note 2 Monitoring|/SEMP/v2/monitor|Querying operational parameters|See note 2    Resources are always nouns, with individual objects being singular and collections being plural.  Objects within a collection are identified by an `obj-id`, which follows the collection name with the form `collection-name/obj-id`.  Actions within an object are identified by an `action-id`, which follows the object name with the form `obj-id/action-id`.  Some examples:  ``` /SEMP/v2/config/msgVpns                        ; MsgVpn collection /SEMP/v2/config/msgVpns/a                      ; MsgVpn object named \"a\" /SEMP/v2/config/msgVpns/a/queues               ; Queue collection in MsgVpn \"a\" /SEMP/v2/config/msgVpns/a/queues/b             ; Queue object named \"b\" in MsgVpn \"a\" /SEMP/v2/action/msgVpns/a/queues/b/startReplay ; Action that starts a replay on Queue \"b\" in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients             ; Client collection in MsgVpn \"a\" /SEMP/v2/monitor/msgVpns/a/clients/c           ; Client object named \"c\" in MsgVpn \"a\" ```  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and are described by JSON arrays. Each item in the array represents an object in the same manner as the individual object would normally be represented. In the configuration API, the creation of a new object is done through its collection resource.  ## Object and Action Resources  Objects are composed of attributes, actions, collections, and other objects. They are described by JSON objects as name/value pairs. The collections and actions of an object are not contained directly in the object's JSON content; rather the content includes an attribute containing a URI which points to the collections and actions. These contained resources must be managed through this URI. At a minimum, every object has one or more identifying attributes, and its own `uri` attribute which contains the URI pointing to itself.  Actions are also composed of attributes, and are described by JSON objects as name/value pairs. Unlike objects, however, they are not members of a collection and cannot be retrieved, only performed. Actions only exist in the action API.  Attributes in an object or action may have any combination of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written.|See note 3 Write-Only|Attribute can only be written, not read, unless the attribute is also opaque|See the documentation for the opaque property Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version| Opaque|Attribute can be set or retrieved in opaque form when the `opaquePassword` query parameter is present|See the `opaquePassword` query parameter documentation    In some requests, certain attributes may only be provided in certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request    In the monitoring API, any non-identifying attribute may not be returned in a GET.  ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these general principles. Note that some methods are only used in certain APIs:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object (see note 5)|New attribute values|Object attributes and metadata|Set to default, with certain exceptions (see note 4) PUT|Action|Performs action|Action arguments|Action metadata|N/A PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many method/URI combinations. Individual URIs may document additional parameters. Note that multiple query parameters can be used together in a single URI, separated by the ampersand character. For example:  ``` ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 ```  ### select  Include in the response only selected attributes of the object, or exclude from the response selected attributes of the object. Use this query parameter to limit the size of the returned data for each returned object, return only those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the list contains attribute names that are not prefaced by `-`, only those attributes are included in the response. If the list contains attribute names that are prefaced by `-`, those attributes are excluded from the response. If the list contains both types, then the difference of the first set of attributes and the second set of attributes is returned. If the list is empty (i.e. `select=`), no attributes are returned.  All attributes that are prefaced by `-` must follow all attributes that are not prefaced by `-`. In addition, each attribute name in the list must match at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute names are supported using periods (e.g. `parentName.childName`).  Some examples:  ``` ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication* ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication* ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission ```  ### where  Include in the response only objects where certain conditions are true. Use this query parameter to limit which objects are returned to those whose attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions must be true for the object to be included in the response. Each expression takes the form:  ``` expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' ```  `value` may be a number, string, `true`, or `false`, as appropriate for the type of `attribute-name`. Greater-than and less-than comparisons only work for numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more characters). Some examples:  ``` ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100 ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* ```  ### count  Limit the count of objects in the response. This can be useful to limit the size of the response for large collections. The minimum value for `count` is `1` and the default is `10`. There is also a per-collection maximum value to limit request handling time. For example:  ``` ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 ```  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data that should not be created or interpreted by SEMP clients, and should only be used as described below.  When a request is made for a collection and there may be additional objects available for retrieval that are not included in the initial response, the response will include a `cursorQuery` field containing a cursor. The value of this field can be specified in the `cursor` query parameter of a subsequent request to retrieve the next page of objects. For convenience, an appropriate URI is constructed automatically by the broker and included in the `nextPageUri` field of the response. This URI can be used directly to retrieve the next page of objects.  ### opaquePassword  Attributes with the opaque property are also write-only and so cannot normally be retrieved in a GET. However, when a password is provided in the `opaquePassword` query parameter, attributes with the opaque property are retrieved in a GET in opaque form, encrypted with this password. The query parameter can also be used on a POST, PATCH, or PUT to set opaque attributes using opaque attribute values retrieved in a GET, so long as:  1. the same password that was used to retrieve the opaque attribute values is provided; and  2. the broker to which the request is being sent has the same major and minor SEMP version as the broker that produced the opaque attribute values.  The password provided in the query parameter must be a minimum of 8 characters and a maximum of 128 characters.  The query parameter can only be used in the configuration API, and only over HTTPS.  ## Help  Visit [our website](https://solace.com) to learn more about Solace.  You can also download the SEMP API specifications by clicking [here](https://solace.com/downloads/).  If you need additional support, please contact us at [support@solace.com](mailto:support@solace.com).  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|On a PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT, except in the following two cases: there is a mutual requires relationship with another non-write-only attribute and both attributes are absent from the request; or the attribute is also opaque and the `opaquePassword` query parameter is provided in the request. 5|On a PUT, if the object does not exist, it is created first.  
 *
 * OpenAPI spec version: 2.19
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;
use std::unimplemented;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct DistributedCacheApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DistributedCacheApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DistributedCacheApiClient<C> {
        DistributedCacheApiClient {
            configuration: configuration,
        }
    }
}

pub trait DistributedCacheApi {
    fn create_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, body: ::models::MsgVpnDistributedCache, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, body: ::models::MsgVpnDistributedCacheCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefix, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterInstance, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterTopic, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, topic_prefix: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, topic_prefix: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_instances(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstancesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, topic: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_cluster_topics(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_cache_clusters(&self, msg_vpn_name: &str, cache_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClustersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_distributed_caches(&self, msg_vpn_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCachesResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, body: ::models::MsgVpnDistributedCache, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, body: ::models::MsgVpnDistributedCacheClusterInstance, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, body: ::models::MsgVpnDistributedCache, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, body: ::models::MsgVpnDistributedCacheClusterInstance, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>DistributedCacheApi for DistributedCacheApiClient<C> {
    fn create_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, body: ::models::MsgVpnDistributedCache, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, body: ::models::MsgVpnDistributedCacheCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefix, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterInstance, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheClusterTopic, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, topic_prefix: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name, topicPrefix=topic_prefix);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, topic=topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefix(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, topic_prefix: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes/{topicPrefix}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name, topicPrefix=topic_prefix);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_cluster_topic_prefixes(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, home_cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters/{homeClusterName}/topicPrefixes?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, homeClusterName=home_cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClusterTopicPrefixesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_global_caching_home_clusters(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/globalCachingHomeClusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterGlobalCachingHomeClustersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_instances(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstancesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstancesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_topic(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, topic: &str, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics/{topic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, topic=topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_cluster_topics(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterTopicsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/topics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterTopicsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_cache_clusters(&self, msg_vpn_name: &str, cache_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClustersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClustersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_distributed_caches(&self, msg_vpn_name: &str, count: i32, cursor: &str, opaque_password: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCachesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &count) != "\"\"" {
                    // println!("count is: {}", format!("{:?}", &count));
                    query.append_pair("count", &count.to_string());
                }


                if format!("{:?}", &cursor) != "\"\"" {
                    // println!("cursor is: {}", format!("{:?}", &cursor));
                    query.append_pair("cursor", &cursor.to_string());
                }


                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &_where) != "\"\"" {
                    // println!("_where is: {}", format!("{:?}", &_where));
                    query.append_pair("where", &_where.join(",").to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCachesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, body: ::models::MsgVpnDistributedCache, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, body: ::models::MsgVpnDistributedCacheClusterInstance, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_distributed_cache(&self, msg_vpn_name: &str, cache_name: &str, body: ::models::MsgVpnDistributedCache, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_distributed_cache_cluster(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, body: ::models::MsgVpnDistributedCacheCluster, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_distributed_cache_cluster_instance(&self, msg_vpn_name: &str, cache_name: &str, cluster_name: &str, instance_name: &str, body: ::models::MsgVpnDistributedCacheClusterInstance, opaque_password: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnDistributedCacheClusterInstanceResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

                if format!("{:?}", &opaque_password) != "\"\"" {
                    // println!("opaque_password is: {}", format!("{:?}", &opaque_password));
                    query.append_pair("opaquePassword", &opaque_password.to_string());
                }


                if format!("{:?}", &select) != "\"\"" {
                    // println!("select is: {}", format!("{:?}", &select));
                    query.append_pair("select", &select.join(",").to_string());
                }

            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/distributedCaches/{cacheName}/clusters/{clusterName}/instances/{instanceName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, cacheName=cache_name, clusterName=cluster_name, instanceName=instance_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnDistributedCacheClusterInstanceResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
