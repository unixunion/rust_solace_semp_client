# MsgVpnAuthenticationOauthProvider

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience_claim_name** | **String** | The audience claim name, indicating which part of the object to use for determining the audience. The default value is &#x60;\&quot;aud\&quot;&#x60;. | [optional] [default to null]
**audience_claim_source** | **String** | The audience claim source, indicating where to search for the audience value. The default value is &#x60;\&quot;id-token\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;access-token\&quot; - The OAuth v2 access_token. \&quot;id-token\&quot; - The OpenID Connect id_token. \&quot;introspection\&quot; - The result of introspecting the OAuth v2 access_token. &lt;/pre&gt;  | [optional] [default to null]
**audience_claim_value** | **String** | The required audience value for a token to be considered valid. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**audience_validation_enabled** | **bool** | Enable or disable audience validation. The default value is &#x60;false&#x60;. | [optional] [default to null]
**authorization_group_claim_name** | **String** | The authorization group claim name, indicating which part of the object to use for determining the authorization group. The default value is &#x60;\&quot;scope\&quot;&#x60;. | [optional] [default to null]
**authorization_group_claim_source** | **String** | The authorization group claim source, indicating where to search for the authorization group name. The default value is &#x60;\&quot;id-token\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;access-token\&quot; - The OAuth v2 access_token. \&quot;id-token\&quot; - The OpenID Connect id_token. \&quot;introspection\&quot; - The result of introspecting the OAuth v2 access_token. &lt;/pre&gt;  | [optional] [default to null]
**authorization_group_enabled** | **bool** | Enable or disable OAuth based authorization. When enabled, the configured authorization type for OAuth clients is overridden. The default value is &#x60;false&#x60;. | [optional] [default to null]
**disconnect_on_token_expiration_enabled** | **bool** | Enable or disable the disconnection of clients when their tokens expire. Changing this value does not affect existing clients, only new client connections. The default value is &#x60;true&#x60;. | [optional] [default to null]
**enabled** | **bool** | Enable or disable OAuth Provider client authentication. The default value is &#x60;false&#x60;. | [optional] [default to null]
**jwks_refresh_interval** | **i32** | The number of seconds between forced JWKS public key refreshing. The default value is &#x60;86400&#x60;. | [optional] [default to null]
**jwks_uri** | **String** | The URI where the OAuth provider publishes its JWKS public keys. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**msg_vpn_name** | **String** | The name of the Message VPN. | [optional] [default to null]
**oauth_provider_name** | **String** | The name of the OAuth Provider. | [optional] [default to null]
**token_ignore_time_limits_enabled** | **bool** | Enable or disable whether to ignore time limits and accept tokens that are not yet valid or are no longer valid. The default value is &#x60;false&#x60;. | [optional] [default to null]
**token_introspection_parameter_name** | **String** | The parameter name used to identify the token during access token introspection. A standards compliant OAuth introspection server expects \&quot;token\&quot;. The default value is &#x60;\&quot;token\&quot;&#x60;. | [optional] [default to null]
**token_introspection_password** | **String** | The password to use when logging into the token introspection URI. This attribute is absent from a GET and not updated when absent in a PUT. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**token_introspection_timeout** | **i32** | The maximum time in seconds a token introspection is allowed to take. The default value is &#x60;1&#x60;. | [optional] [default to null]
**token_introspection_uri** | **String** | The token introspection URI of the OAuth authentication server. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**token_introspection_username** | **String** | The username to use when logging into the token introspection URI. The default value is &#x60;\&quot;\&quot;&#x60;. | [optional] [default to null]
**username_claim_name** | **String** | The username claim name, indicating which part of the object to use for determining the username. The default value is &#x60;\&quot;sub\&quot;&#x60;. | [optional] [default to null]
**username_claim_source** | **String** | The username claim source, indicating where to search for the username value. The default value is &#x60;\&quot;id-token\&quot;&#x60;. The allowed values and their meaning are:  &lt;pre&gt; \&quot;access-token\&quot; - The OAuth v2 access_token. \&quot;id-token\&quot; - The OpenID Connect id_token. \&quot;introspection\&quot; - The result of introspecting the OAuth v2 access_token. &lt;/pre&gt;  | [optional] [default to null]
**username_validate_enabled** | **bool** | Enable or disable whether the API provided username will be validated against the username calculated from the token(s); the connection attempt is rejected if they differ. The default value is &#x60;false&#x60;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


