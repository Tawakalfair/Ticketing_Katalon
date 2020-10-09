<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Put Client-Products</name>
   <tag></tag>
   <elementGuidId>555e9be0-cc0b-4a75-b87b-04c2fc3a5972</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;id\&quot;:\&quot;${id}\&quot;,  \n\&quot;idProduct\&quot;:{\n\&quot;id\&quot;:\&quot;${idProduct}\&quot;\n},\n\&quot;idCompany\&quot;:{\n\&quot;id\&quot;:\&quot;${idCompany}\&quot;\n},\n\&quot;ticketUrgent\&quot;:\&quot;${ticketUrgent}\&quot;,\n\&quot;ticketMedium\&quot;:\&quot;${ticketMedium}\&quot;\n  \n}&quot;,
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
   <restUrl>${GlobalVariable.baseUrl}/client-products/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'d420a688-e41f-474c-aed2-71bb0a3bea4b'</defaultValue>
      <description></description>
      <id>edef345d-d0f9-4f1c-af48-43f7592e69e7</id>
      <masked>false</masked>
      <name>idProduct</name>
   </variables>
   <variables>
      <defaultValue>'8eac4724-c936-45d1-a4a7-080bf89b261a'</defaultValue>
      <description></description>
      <id>d26fe61b-6aa1-450f-b721-9f4503f35981</id>
      <masked>false</masked>
      <name>idCompany</name>
   </variables>
   <variables>
      <defaultValue>'415be140-70f8-43b1-8141-73506133f45b'</defaultValue>
      <description></description>
      <id>f9618af4-de77-4c51-bc54-03f0af193b9f</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>4</defaultValue>
      <description></description>
      <id>00e4683c-75b4-4649-a7ac-857ebdd2c8f8</id>
      <masked>false</masked>
      <name>ticketUrgent</name>
   </variables>
   <variables>
      <defaultValue>5</defaultValue>
      <description></description>
      <id>3af51e56-6296-4eb1-b908-9a606e1ed4f7</id>
      <masked>false</masked>
      <name>ticketMedium</name>
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
