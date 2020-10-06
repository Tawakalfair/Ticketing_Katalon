<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>insertAgentRelations</name>
   <tag></tag>
   <elementGuidId>3d5d8a54-c5a2-43cf-ad5a-70d4bdf812dc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;startDate\&quot; : \&quot;${start}\&quot;,\n  \&quot;endDate\&quot; : \&quot;${end}\&quot;,\n  \&quot;idAgent\&quot; : {\n  \t\&quot;nip\&quot; : \&quot;${nip}\&quot;\n  },\n  \&quot;idCompany\&quot; : {\n  \t\&quot;code\&quot; : \&quot;${companyCode}\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
   <restUrl>http://localhost:8080/agent-relations/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'m'</defaultValue>
      <description></description>
      <id>37576a87-8ae1-45dd-84be-e6eb78a37f0d</id>
      <masked>false</masked>
      <name>companyCode</name>
   </variables>
   <variables>
      <defaultValue>'22'</defaultValue>
      <description></description>
      <id>9c76a979-0c18-4e16-a316-a0fe8f8995f6</id>
      <masked>false</masked>
      <name>nip</name>
   </variables>
   <variables>
      <defaultValue>'2020-09-14'</defaultValue>
      <description></description>
      <id>8d5eba67-f118-423f-b07e-68020d191ae8</id>
      <masked>false</masked>
      <name>start</name>
   </variables>
   <variables>
      <defaultValue>'2020-12-14'</defaultValue>
      <description></description>
      <id>b8db4e04-fab2-4b4b-bffd-5f8bc4491913</id>
      <masked>false</masked>
      <name>end</name>
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
