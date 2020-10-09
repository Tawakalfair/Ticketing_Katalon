<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Users</name>
   <tag></tag>
   <elementGuidId>7dced04f-4e9f-4df6-a07b-3d2d477789d1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;nip\&quot;:\&quot;${nip}\&quot;,\n\&quot;name\&quot;:\&quot;${name}\&quot;,\n\&quot;address\&quot;:\&quot;${address}\&quot;,\n\&quot;contact\&quot;:\&quot;${contact}\&quot;,\n\&quot;idRole\&quot;:{\n\&quot;code\&quot;:\&quot;${roleCode}\&quot;\n},\n\&quot;idCompany\&quot;:{\n\&quot;code\&quot;:\&quot;${companyCode}\&quot;\n}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
   </httpHeaderProperties>
   <katalonVersion>7.7.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/users/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'06969696969'</defaultValue>
      <description></description>
      <id>046f905d-b9cc-4b36-8cdc-35d6c3d4b869</id>
      <masked>false</masked>
      <name>nip</name>
   </variables>
   <variables>
      <defaultValue>'Chernobyl RT 01/04'</defaultValue>
      <description></description>
      <id>e7f4ce60-4928-404f-a4ec-78d0d077ec5b</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>'Mang Jon'</defaultValue>
      <description></description>
      <id>29a52d0d-4fcc-4196-aa93-b8225bf05f5b</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'1292938499'</defaultValue>
      <description></description>
      <id>56d91a3d-110c-44e9-9aef-aa205027f64a</id>
      <masked>false</masked>
      <name>contact</name>
   </variables>
   <variables>
      <defaultValue>'CTM'</defaultValue>
      <description></description>
      <id>523fcb44-e51e-4fda-9f1e-e834b9e55a5a</id>
      <masked>false</masked>
      <name>roleCode</name>
   </variables>
   <variables>
      <defaultValue>'f'</defaultValue>
      <description></description>
      <id>54f4d082-e53d-4838-b0ce-a765af9165f6</id>
      <masked>false</masked>
      <name>companyCode</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
