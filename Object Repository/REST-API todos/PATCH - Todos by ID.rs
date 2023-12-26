<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PATCH - Todos by ID</name>
   <tag></tag>
   <elementGuidId>dc510f7c-dab7-4294-b40c-6c2ceab2b647</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;title\&quot; : \&quot;${title}\&quot;,\n  \t\&quot;completed\&quot; : ${completed}\n}&quot;,
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
      <webElementGuid>291cc4ce-850e-4099-967d-869f64a21b2c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${baseurl}/todos/${id}</restUrl>
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
      <id>962f2df5-5e7b-451f-b1ad-4e409d2ceaef</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>fffd6bf6-53dc-48bf-bfec-a672ef8ad077</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'Assignment 2 REST-API'</defaultValue>
      <description></description>
      <id>3fc379e2-07b3-421d-af7a-f5cbf80b4812</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>90e8539c-213d-415c-b0f4-b337f3f66c39</id>
      <masked>false</masked>
      <name>completed</name>
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
