<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Account</name>
   <tag></tag>
   <elementGuidId>64c22f5b-128d-4233-9bfb-aeb2fd954d02</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{    \n    \n        \&quot;pass\&quot;:\&quot;${oldPass}\&quot;\n    ,\n    \&quot;idAccount\&quot;:{\n        \&quot;id\&quot;:\&quot;${idAccount}\&quot;,\n        \&quot;email\&quot; : \&quot;${email}\&quot;,\n        \&quot;pass\&quot;:\&quot;${newPass}\&quot;,\n        \&quot;idUser\&quot; : {\n            \&quot;id\&quot;:\&quot;${idUser}\&quot;,\n            \&quot;name\&quot; :\&quot;${name}\&quot;,\n            \&quot;nip\&quot; : \&quot;${nip}\&quot;,\n            \&quot;idRole\&quot; : {\n                \&quot;id\&quot;:\&quot;${idRole}\&quot;,\n                \&quot;code\&quot; : \&quot;${code}\&quot;\n            },\n            \&quot;idCompany\&quot; : {\n                \&quot;id\&quot;:\&quot;${idCompany}\&quot;,\n                \&quot;name\&quot; : \&quot;${companyName}\&quot;,\n                \&quot;code\&quot; : \&quot;${companyCode}\&quot;\n            }\n        }\n    }}&quot;,
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
   <restUrl>${GlobalVariable.baseUrl}/accounts/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'tes123'</defaultValue>
      <description></description>
      <id>23a129a1-8e97-430e-8da4-aeb19bd74a2f</id>
      <masked>false</masked>
      <name>oldPass</name>
   </variables>
   <variables>
      <defaultValue>'b6a73e7c-be8e-47fb-851d-49509fae28a9'</defaultValue>
      <description></description>
      <id>1f5d46fa-0494-4ead-9da9-3d5458e25b46</id>
      <masked>false</masked>
      <name>idAccount</name>
   </variables>
   <variables>
      <defaultValue>'anastasiusmanurung@gmail.com'</defaultValue>
      <description></description>
      <id>b0bd29f0-6dc8-4d53-ad64-dab85dd5e7b9</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'tes123'</defaultValue>
      <description></description>
      <id>89f53b36-94fa-4327-9395-d20b53d93979</id>
      <masked>false</masked>
      <name>newPass</name>
   </variables>
   <variables>
      <defaultValue>'64e7cbc9-30d7-4d02-9046-6dafa0112a48'</defaultValue>
      <description></description>
      <id>bf18af62-85ab-4311-84d4-d1ed3f49ae23</id>
      <masked>false</masked>
      <name>idUser</name>
   </variables>
   <variables>
      <defaultValue>'anas'</defaultValue>
      <description></description>
      <id>33ecc326-0e4a-4f43-8441-8a91c99f28ce</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'1234'</defaultValue>
      <description></description>
      <id>6a0b1c2d-7fb7-4939-8f95-ebbd282fdf87</id>
      <masked>false</masked>
      <name>nip</name>
   </variables>
   <variables>
      <defaultValue>'26dfb4b3-583d-4615-bef4-e0abae034951'</defaultValue>
      <description></description>
      <id>18fd587b-7c0b-4090-8ddd-99c55f3534b2</id>
      <masked>false</masked>
      <name>idRole</name>
   </variables>
   <variables>
      <defaultValue>'AGT'</defaultValue>
      <description></description>
      <id>476c6a40-9822-4985-8726-ffea6da24405</id>
      <masked>false</masked>
      <name>code</name>
   </variables>
   <variables>
      <defaultValue>'94077080-9835-42a0-aeaf-9e0dc5a9b8b5'</defaultValue>
      <description></description>
      <id>ab90a489-fc47-4f47-8df4-f0fa4763e73e</id>
      <masked>false</masked>
      <name>idCompany</name>
   </variables>
   <variables>
      <defaultValue>'Karuhun'</defaultValue>
      <description></description>
      <id>638b2756-2676-437b-9a81-b2b0b9ff8769</id>
      <masked>false</masked>
      <name>companyName</name>
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
