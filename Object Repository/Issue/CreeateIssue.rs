<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreeateIssue</name>
   <tag></tag>
   <elementGuidId>4558b709-4fa1-4436-a3d2-995419a6f676</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;fields\&quot;: {\n       \&quot;project\&quot;:\n       {\n          \&quot;key\&quot;: \&quot;BTOE\&quot;\n       },\n       \&quot;summary\&quot;: \&quot;Create issue Via REST API\&quot;,\n       \&quot;description\&quot;: \&quot;Note: Note: \\nCustomer:Orange Business Services\\nLink to Ticket documents https://ac-a418742.documents.em2.oraclecloud.com/documents/link/LF6438ADAFFC4E01B0ABD97BD634D7CE7D67F7B218A2/folder/F5EC35FACC02AD1A0F8DB436D634D7CE7D67F7B218A2/_180924-000152\&quot;,\n       \&quot;customfield_10107\&quot;:{ \&quot;value\&quot;: \&quot;FieldBug\&quot;},\n       \&quot;customfield_10113\&quot;:[{\&quot;value\&quot;:\&quot;None\&quot;}],\n       \&quot;customfield_10114\&quot;: \&quot;3.1.2.89\&quot;,\n       \&quot;customfield_10102\&quot;: {\n      \&quot;value\&quot;: \&quot;Functional\&quot;\n    },\n    \&quot;customfield_10803\&quot;: \&quot;AudioCodes Test\&quot;,\n    \&quot;customfield_10601\&quot;: \&quot;2705\&quot;,\n    \&quot;customfield_10509\&quot;: \&quot;180924-000152\&quot;,\n        \&quot;assignee\&quot;: null,\n            \&quot;reporter\&quot;: {\n      \&quot;name\&quot;: \&quot;DanielSh\&quot;\n    },\n    \&quot;components\&quot;: [\n      {\n        \&quot;name\&quot;: \&quot;ETAS\&quot;\n      }\n    ],\n    \&quot;versions\&quot;: [\n      {\n        \&quot;name\&quot;: \&quot;BTOE2.0.3\&quot;\n      }\n      ],\n          \&quot;priority\&quot;: {\n      \&quot;name\&quot;: \&quot;Medium\&quot;\n    },\n       \&quot;issuetype\&quot;: {\n          \&quot;name\&quot;: \&quot;Bug\&quot;\n       }\n    }\n}\n&quot;,
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
      <value>Basic bWljaGFlbHNoOlJpY2gxOTg2</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/issue</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'https://test-acjira/rest/api/2'</defaultValue>
      <description></description>
      <id>1dbef7c5-0d9e-4faa-b470-aa1ef244d01d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>35017923-674e-405d-9698-3c366a0795d3</id>
      <masked>false</masked>
      <name>variable_0</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c938f58d-88a2-4900-95be-7bdfbe8690ab</id>
      <masked>false</masked>
      <name>variable_1</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
