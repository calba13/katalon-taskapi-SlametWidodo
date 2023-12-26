<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - Add New User</name>
   <tag></tag>
   <elementGuidId>86dce564-8015-4d9c-9fd2-8a31c24b95f6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;${name}\&quot;,\n  \&quot;username\&quot;: \&quot;${username}\&quot;,\n  \&quot;email\&quot;: \&quot;${email}\&quot;,\n  \&quot;phone\&quot;: \&quot;${phone}\&quot;,\n  \&quot;company\&quot;: [{\n    \&quot;name\&quot;: \&quot;${compname}\&quot;\n  }],\n  \&quot;training\&quot;: [{\n    \&quot;title\&quot; : \&quot;${title}\&quot;,\n    \&quot;building\&quot; : \&quot;${building}\&quot;,\n    \&quot;street\&quot;: \&quot;${street}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;geo\&quot;: {\n      \&quot;lat\&quot;: \&quot;${lat}\&quot;,\n      \&quot;lng\&quot;: \&quot;${lng}\&quot;\n    }\n  }]\n}&quot;,
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
      <webElementGuid>b3724a84-55c7-45a6-b0d1-eee176794111</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseurl}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>191bb542-f08a-4a56-b0e0-00fa2328b9d7</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <variables>
      <defaultValue>'DODO - Hacktiv8'</defaultValue>
      <description></description>
      <id>5f6b241c-56d9-496c-be5d-1385216517e5</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'Hacktiv8.MRQC'</defaultValue>
      <description></description>
      <id>e7d52ff6-952e-4506-8ad4-05933d7425f3</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'hacktiv8@april.biz'</defaultValue>
      <description></description>
      <id>8e483b96-f08c-4bd5-b3b3-42a1110c1074</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'1-770-736-8031 x56442'</defaultValue>
      <description></description>
      <id>035926b9-5694-416b-8f5e-072eeaf12f75</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>'OTO Multiartha'</defaultValue>
      <description></description>
      <id>c1f710ec-2fde-4793-b013-2fffe41e45a8</id>
      <masked>false</masked>
      <name>compname</name>
   </variables>
   <variables>
      <defaultValue>'Automation Katalon'</defaultValue>
      <description></description>
      <id>7a85f3a1-c8ce-4159-9dcb-c1dcd53623b6</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'Hacktiv8'</defaultValue>
      <description></description>
      <id>9f777fc8-6638-405e-890e-3f9484322fb7</id>
      <masked>false</masked>
      <name>building</name>
   </variables>
   <variables>
      <defaultValue>'Pondok Indah'</defaultValue>
      <description></description>
      <id>63e5c4f7-b946-4452-ac2d-657d1eb87e9e</id>
      <masked>false</masked>
      <name>street</name>
   </variables>
   <variables>
      <defaultValue>'Jakarta Selatan'</defaultValue>
      <description></description>
      <id>14741a19-e730-43a2-85d9-fea57be880ca</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>'-37.3159'</defaultValue>
      <description></description>
      <id>a8bf0bd5-aec2-459a-949d-ba5c26dbb931</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>'81.1496'</defaultValue>
      <description></description>
      <id>de007126-1b45-4f86-ab36-fef1a5e27654</id>
      <masked>false</masked>
      <name>lng</name>
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
