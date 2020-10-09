<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Put Users</name>
   <tag></tag>
   <elementGuidId>a065e289-325f-4619-9b4b-6e931cebd14a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;file&quot;,
      &quot;value&quot;: &quot;C:\\Users\\Tawakal\\Downloads\\283481666012211.png&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;users&quot;,
      &quot;value&quot;: &quot;{\&quot;id\&quot;:\&quot;${id}\&quot;,\&quot;nip\&quot;:\&quot;${nip}\&quot;,\&quot;name\&quot;:\&quot;${name}\&quot;,\&quot;address\&quot;:\&quot;${address}\&quot;,\&quot;contact\&quot;:\&quot;${contact}\&quot;,\&quot;idRole\&quot;:{\&quot;code\&quot;:\&quot;${roleCode}\&quot;},\&quot;idCompany\&quot;:{\&quot;code\&quot;:\&quot;${companyCode}\&quot;}}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;application/json&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <katalonVersion>7.7.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
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
      <defaultValue>'1234'</defaultValue>
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
      <defaultValue>'Mang Jony'</defaultValue>
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
   <variables>
      <defaultValue>'97d4d065-c0bb-4526-adab-7367dfcff5c4'</defaultValue>
      <description></description>
      <id>f56258fc-ca61-4c88-b05c-596adc34ddfc</id>
      <masked>false</masked>
      <name>id</name>
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
