import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('Tickets/Update', [('subject') : 'Bug Jancuk', ('ticketId') : 'acc4219c-bcea-43bd-a14f-5666796aa289'
            , ('ticketCode') : 'Tpsd-732', ('ticketVer') : 24, ('ticketAct') : true, ('ticketCB') : 'Customer Bootcamp', ('ticketUB') : 'Customer Bootcamp'
            , ('ticketCA') : '2020-09-29T09:04:56.472+00:00', ('custId') : '20e9f791-8dbb-449b-b4b2-8cac4bcc0b3a', ('classId') : '38c26646-ae04-4a52-8d5f-283b697e1aa1'
            , ('priorityId') : '3bfbcd83-0f41-439c-9939-1e0ff8c10aeb', ('productId') : 'd420a688-e41f-474c-aed2-71bb0a3bea4b'
            , ('statusCode') : 'RO']))

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('Tickets/Get by Code', [('code') : 'Tpsd-732']))

WS.verifyElementPropertyValue(response, 'idTicket.id', ticketId)

WS.verifyElementPropertyValue(response, 'idTicket.code', ticketCode)

WS.verifyElementPropertyValue(response, 'idTicket.subject', subject)

WS.verifyElementPropertyValue(response, 'idTicket.idStatus.code', statusCode)

