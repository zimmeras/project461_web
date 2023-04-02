# Ece461Spring2023Project2.PackageData

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **String** | Package contents. This is the zip file uploaded by the user. (Encoded as text using a Base64 encoding).  This will be a zipped version of an npm package&#x27;s GitHub repository, minus the \&quot;.git/\&quot; directory.\&quot; It will, for example, include the \&quot;package.json\&quot; file that can be used to retrieve the project homepage.  See https://docs.npmjs.com/cli/v7/configuring-npm/package-json#homepage. | [optional] 
**URL** | **String** | Package URL (for use in public ingest). | [optional] 
**jSProgram** | **String** | A JavaScript program (for use with sensitive modules). | [optional] 
