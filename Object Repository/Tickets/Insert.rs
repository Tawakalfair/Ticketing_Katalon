<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Insert</name>
   <tag></tag>
   <elementGuidId>3c5ff37b-b37f-40cc-8d28-0eeb480a59ca</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;subject\&quot; : \&quot;${subject}\&quot;,\n  \t\&quot;createdBy\&quot; : \&quot;${createdBy}\&quot;,\n    \&quot;idCustomer\&quot; : {\n        \&quot;nip\&quot; : \&quot;${nip}\&quot;\n    },\n    \&quot;idClassification\&quot; : {\n        \&quot;code\&quot; : \&quot;${codeClass}\&quot;\n    },\n    \&quot;idStatus\&quot; : {\n        \&quot;code\&quot; : \&quot;${codeStatus}\&quot;\n    },\n    \&quot;idPriority\&quot; : {\n        \&quot;code\&quot; : \&quot;${codePriority}\&quot;\n    },\n    \&quot;idProduct\&quot; : {\n        \&quot;code\&quot; : \&quot;${codeProduct}\&quot;\n    }\n}&quot;,
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
   <restUrl>http://localhost:8080/tickets/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Error Cok'</defaultValue>
      <description></description>
      <id>a7687900-d450-4bff-87a3-25fdf437a2b4</id>
      <masked>false</masked>
      <name>subject</name>
   </variables>
   <variables>
      <defaultValue>'11111111'</defaultValue>
      <description></description>
      <id>e205712a-5f05-4576-895a-66542cdc4aa7</id>
      <masked>false</masked>
      <name>nip</name>
   </variables>
   <variables>
      <defaultValue>'ERR'</defaultValue>
      <description></description>
      <id>364e8f7a-f826-41a4-8d9a-c615d88ab863</id>
      <masked>false</masked>
      <name>codeClass</name>
   </variables>
   <variables>
      <defaultValue>'NOR'</defaultValue>
      <description></description>
      <id>0284e99a-f268-4256-9f16-c58c02385743</id>
      <masked>false</masked>
      <name>codePriority</name>
   </variables>
   <variables>
      <defaultValue>'OP'</defaultValue>
      <description></description>
      <id>15c5991b-53ba-4eb7-a297-e3d0bbe39fdc</id>
      <masked>false</masked>
      <name>codeStatus</name>
   </variables>
   <variables>
      <defaultValue>'BTK'</defaultValue>
      <description></description>
      <id>43e7fcca-2b02-452c-b376-5d0afe4ef33c</id>
      <masked>false</masked>
      <name>codeProduct</name>
   </variables>
   <variables>
      <defaultValue>'Customer Bootcamp'</defaultValue>
      <description></description>
      <id>289e50e3-bcfa-4515-be0a-42e04f87ccb8</id>
      <masked>false</masked>
      <name>createdBy</name>
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
