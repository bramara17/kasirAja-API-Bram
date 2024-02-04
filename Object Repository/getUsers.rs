<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getUsers</name>
   <tag></tag>
   <elementGuidId>cff9af09-b911-49dd-af02-237d90c19c92</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b7b71c27-26c0-4703-b21f-ec3cfc1a35ae</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURLKasirAja</defaultValue>
      <description></description>
      <id>29a069bd-047e-4865-a112-1bfa91057768</id>
      <masked>false</masked>
      <name>URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.email1</defaultValue>
      <description></description>
      <id>69b28bf0-725a-4b37-b0bc-675bca7ac701</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.pswd1</defaultValue>
      <description></description>
      <id>54137d2e-1074-4b1d-b06f-562f4bf49490</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accessTokenKasirAja</defaultValue>
      <description></description>
      <id>17ed441e-7d9c-4f68-b84c-023242739a0c</id>
      <masked>false</masked>
      <name>token</name>
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
