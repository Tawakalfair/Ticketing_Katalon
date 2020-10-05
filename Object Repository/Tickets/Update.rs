<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update</name>
   <tag></tag>
   <elementGuidId>bf2ee6de-d3d0-44ad-a10d-3e7666ae3293</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${ticketId}\&quot;,\n    \&quot;createdAt\&quot;: \&quot;${ticketCA}\&quot;,\n    \&quot;createdBy\&quot;: \&quot;${ticketCB}\&quot;,\n    \&quot;updatedBy\&quot;: \&quot;${ticketUB}\&quot;,\n    \&quot;version\&quot;: \&quot;${ticketVer}\&quot;,\n    \&quot;active\&quot;: \&quot;${ticketAct}\&quot;,\n    \&quot;idCustomer\&quot;: {\n      \&quot;id\&quot;: \&quot;${custId}\&quot;\n    },\n    \&quot;idProduct\&quot;: {\n      \&quot;id\&quot;: \&quot;${productId}\&quot;\n    },\n    \&quot;idPriority\&quot;: {\n      \&quot;id\&quot;: \&quot;${priorityId}\&quot;\n    },\n    \&quot;idClassification\&quot;: {\n      \&quot;id\&quot;: \&quot;${classId}\&quot;\n    },\n    \&quot;idStatus\&quot;: {\n      \&quot;code\&quot;: \&quot;${statusCode}\&quot;\n    },\n    \&quot;code\&quot;: \&quot;${ticketCode}\&quot;,\n    \&quot;subject\&quot;: \&quot;${subject}\&quot;\n}&quot;,
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
   <restRequestMethod>PUT</restRequestMethod>
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
      <defaultValue>'Bug Jancuk'</defaultValue>
      <description></description>
      <id>a7687900-d450-4bff-87a3-25fdf437a2b4</id>
      <masked>false</masked>
      <name>subject</name>
   </variables>
   <variables>
      <defaultValue>'acc4219c-bcea-43bd-a14f-5666796aa289'</defaultValue>
      <description></description>
      <id>074561cc-4b13-4817-837f-bac41677a1d3</id>
      <masked>false</masked>
      <name>ticketId</name>
   </variables>
   <variables>
      <defaultValue>'Tpsd-732'</defaultValue>
      <description></description>
      <id>3a62777d-60bd-4dd5-9c6e-ec29ca1cbab1</id>
      <masked>false</masked>
      <name>ticketCode</name>
   </variables>
   <variables>
      <defaultValue>16</defaultValue>
      <description></description>
      <id>72968b70-98e6-4ceb-a262-bac92c9ea824</id>
      <masked>false</masked>
      <name>ticketVer</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>5b8d7630-6ae0-4195-87d4-360af5fc1355</id>
      <masked>false</masked>
      <name>ticketAct</name>
   </variables>
   <variables>
      <defaultValue>'Customer Bootcamp'</defaultValue>
      <description></description>
      <id>5d9c7359-cece-4092-a01e-2471e3ccf15e</id>
      <masked>false</masked>
      <name>ticketCB</name>
   </variables>
   <variables>
      <defaultValue>'Customer Bootcamp'</defaultValue>
      <description></description>
      <id>698b10fe-0339-48ed-9c03-56b41eeaeeff</id>
      <masked>false</masked>
      <name>ticketUB</name>
   </variables>
   <variables>
      <defaultValue>'2020-09-29T09:04:56.472+00:00'</defaultValue>
      <description></description>
      <id>5d9c7359-cece-4092-a01e-2471e3ccf15e</id>
      <masked>false</masked>
      <name>ticketCA</name>
   </variables>
   <variables>
      <defaultValue>'20e9f791-8dbb-449b-b4b2-8cac4bcc0b3a'</defaultValue>
      <description></description>
      <id>074561cc-4b13-4817-837f-bac41677a1d3</id>
      <masked>false</masked>
      <name>custId</name>
   </variables>
   <variables>
      <defaultValue>'38c26646-ae04-4a52-8d5f-283b697e1aa1'</defaultValue>
      <description></description>
      <id>074561cc-4b13-4817-837f-bac41677a1d3</id>
      <masked>false</masked>
      <name>classId</name>
   </variables>
   <variables>
      <defaultValue>'3bfbcd83-0f41-439c-9939-1e0ff8c10aeb'</defaultValue>
      <description></description>
      <id>074561cc-4b13-4817-837f-bac41677a1d3</id>
      <masked>false</masked>
      <name>priorityId</name>
   </variables>
   <variables>
      <defaultValue>'d420a688-e41f-474c-aed2-71bb0a3bea4b'</defaultValue>
      <description></description>
      <id>074561cc-4b13-4817-837f-bac41677a1d3</id>
      <masked>false</masked>
      <name>productId</name>
   </variables>
   <variables>
      <defaultValue>'RO'</defaultValue>
      <description></description>
      <id>15c5991b-53ba-4eb7-a297-e3d0bbe39fdc</id>
      <masked>false</masked>
      <name>statusCode</name>
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
