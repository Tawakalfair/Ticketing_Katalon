<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Account</name>
   <tag></tag>
   <elementGuidId>06b8154f-b2d7-4d0f-92ed-f1a082501ac1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;email\&quot; : \&quot;${email}\&quot;,\n    \&quot;idUser\&quot; : {\n        \&quot;name\&quot; : \&quot;${username}\&quot;,\n        \&quot;nip\&quot; : \&quot;${nip}\&quot;,\n        \&quot;idRole\&quot; : {\n            \&quot;code\&quot; : \&quot;${rolecode}\&quot;\n        },\n        \&quot;idCompany\&quot; : {\n            \&quot;name\&quot; : \&quot;${companyname}\&quot;,\n          \t\&quot;code\&quot; : \&quot;${companycode}\&quot;\n        }\n    }\n}&quot;,
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
   <restUrl>http://localhost:8080/accounts/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'anastasiusmanurung@gmail.com'</defaultValue>
      <description></description>
      <id>83966594-f502-4ced-8da0-91584cff0e1f</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'gibran'</defaultValue>
      <description></description>
      <id>b7079202-386c-4fcd-a53b-9a5c79c4ad78</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'1234'</defaultValue>
      <description></description>
      <id>e7794af1-3544-4253-9365-985c3208ed33</id>
      <masked>false</masked>
      <name>nip</name>
   </variables>
   <variables>
      <defaultValue>'AGT'</defaultValue>
      <description></description>
      <id>b07a4629-88f1-41ba-803d-69407e2176f1</id>
      <masked>false</masked>
      <name>rolecode</name>
   </variables>
   <variables>
      <defaultValue>'Karuhun'</defaultValue>
      <description></description>
      <id>a773c2e0-555e-49c7-8e45-639377e114c2</id>
      <masked>false</masked>
      <name>companyname</name>
   </variables>
   <variables>
      <defaultValue>'KRHN'</defaultValue>
      <description></description>
      <id>b5484ae3-c84e-4712-ac68-ae82f5c31d89</id>
      <masked>false</masked>
      <name>companycode</name>
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
