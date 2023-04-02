# Ece461Spring2023Project2.DefaultApi

All URIs are relative to */*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createAuthToken**](DefaultApi.md#createAuthToken) | **PUT** /authenticate | 
[**packageByNameDelete**](DefaultApi.md#packageByNameDelete) | **DELETE** /package/byName/{name} | Delete all versions of this package.
[**packageByNameGet**](DefaultApi.md#packageByNameGet) | **GET** /package/byName/{name} | 
[**packageByRegExGet**](DefaultApi.md#packageByRegExGet) | **POST** /package/byRegEx/{regex} | Get any packages fitting the regular expression.
[**packageCreate**](DefaultApi.md#packageCreate) | **POST** /package | 
[**packageDelete**](DefaultApi.md#packageDelete) | **DELETE** /package/{id} | Delete this version of the package.
[**packageRate**](DefaultApi.md#packageRate) | **GET** /package/{id}/rate | 
[**packageRetrieve**](DefaultApi.md#packageRetrieve) | **GET** /package/{id} | Interact with the package with this ID
[**packageUpdate**](DefaultApi.md#packageUpdate) | **PUT** /package/{id} | Update this content of the package.
[**packagesList**](DefaultApi.md#packagesList) | **POST** /packages | Get the packages from the registry.
[**registryReset**](DefaultApi.md#registryReset) | **DELETE** /reset | Reset the registry

<a name="createAuthToken"></a>
# **createAuthToken**
> AuthenticationToken createAuthToken(body)



Create an access token.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let body = new Ece461Spring2023Project2.AuthenticationRequest(); // AuthenticationRequest | 

apiInstance.createAuthToken(body, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**AuthenticationRequest**](AuthenticationRequest.md)|  | 

### Return type

[**AuthenticationToken**](AuthenticationToken.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a name="packageByNameDelete"></a>
# **packageByNameDelete**
> packageByNameDelete(name, opts)

Delete all versions of this package.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let name = new Ece461Spring2023Project2.PackageName(); // PackageName | 
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageByNameDelete(name, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully.');
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | [**PackageName**](.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

<a name="packageByNameGet"></a>
# **packageByNameGet**
> [PackageHistoryEntry] packageByNameGet(name, opts)



Return the history of this package (all versions).

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let name = new Ece461Spring2023Project2.PackageName(); // PackageName | 
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageByNameGet(name, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | [**PackageName**](.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

[**[PackageHistoryEntry]**](PackageHistoryEntry.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a name="packageByRegExGet"></a>
# **packageByRegExGet**
> [PackageMetadata] packageByRegExGet(body, regex, opts)

Get any packages fitting the regular expression.

Search for a package using regular expression over package names and READMEs. This is similar to search by name.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let body = "body_example"; // String | 
let regex = new Ece461Spring2023Project2.PackageRegEx(); // PackageRegEx | 
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageByRegExGet(body, regex, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**String**](String.md)|  | 
 **regex** | [**PackageRegEx**](.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

[**[PackageMetadata]**](PackageMetadata.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a name="packageCreate"></a>
# **packageCreate**
> ModelPackage packageCreate(body, xAuthorization)



### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let body = new Ece461Spring2023Project2.PackageData(); // PackageData | 
let xAuthorization = new Ece461Spring2023Project2.AuthenticationToken(); // AuthenticationToken | 

apiInstance.packageCreate(body, xAuthorization, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**PackageData**](PackageData.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | 

### Return type

[**ModelPackage**](ModelPackage.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a name="packageDelete"></a>
# **packageDelete**
> packageDelete(id, opts)

Delete this version of the package.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let id = new Ece461Spring2023Project2.PackageID(); // PackageID | Package ID
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageDelete(id, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully.');
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**PackageID**](.md)| Package ID | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

<a name="packageRate"></a>
# **packageRate**
> PackageRating packageRate(id, opts)



### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let id = new Ece461Spring2023Project2.PackageID(); // PackageID | 
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageRate(id, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**PackageID**](.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

[**PackageRating**](PackageRating.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a name="packageRetrieve"></a>
# **packageRetrieve**
> ModelPackage packageRetrieve(id, opts)

Interact with the package with this ID

Return this package.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let id = new Ece461Spring2023Project2.PackageID(); // PackageID | ID of package to fetch
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageRetrieve(id, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**PackageID**](.md)| ID of package to fetch | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

[**ModelPackage**](ModelPackage.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a name="packageUpdate"></a>
# **packageUpdate**
> packageUpdate(body, id, opts)

Update this content of the package.

The name, version, and ID must match.  The package contents (from PackageData) will replace the previous contents.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let body = new Ece461Spring2023Project2.ModelPackage(); // ModelPackage | 
let id = new Ece461Spring2023Project2.PackageID(); // PackageID | 
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.packageUpdate(body, id, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully.');
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**ModelPackage**](ModelPackage.md)|  | 
 **id** | [**PackageID**](.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

<a name="packagesList"></a>
# **packagesList**
> [PackageMetadata] packagesList(body, opts)

Get the packages from the registry.

Get any packages fitting the query. Search for packages satisfying the indicated query.  If you want to enumerate all packages, provide an array with a single PackageQuery whose name is \&quot;*\&quot;.  The response is paginated; the response header includes the offset to use in the next query.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let body = [new Ece461Spring2023Project2.PackageQuery()]; // [PackageQuery] | 
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken(), // AuthenticationToken | 
  'offset': new Ece461Spring2023Project2.EnumerateOffset() // EnumerateOffset | Provide this for pagination. If not provided, returns the first page of results.
};
apiInstance.packagesList(body, opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**[PackageQuery]**](PackageQuery.md)|  | 
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 
 **offset** | [**EnumerateOffset**](.md)| Provide this for pagination. If not provided, returns the first page of results. | [optional] 

### Return type

[**[PackageMetadata]**](PackageMetadata.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a name="registryReset"></a>
# **registryReset**
> registryReset(opts)

Reset the registry

Reset the registry to a system default state.

### Example
```javascript
import {Ece461Spring2023Project2} from 'ece_461___spring_2023___project_2';

let apiInstance = new Ece461Spring2023Project2.DefaultApi();
let opts = { 
  'xAuthorization': new Ece461Spring2023Project2.AuthenticationToken() // AuthenticationToken | 
};
apiInstance.registryReset(opts, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully.');
  }
});
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **xAuthorization** | [**AuthenticationToken**](.md)|  | [optional] 

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

