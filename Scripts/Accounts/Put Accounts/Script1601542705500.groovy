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

response = WS.sendRequest(findTestObject('Accounts/Update Account', [('oldPass') : 'tes123', ('idAccount') : 'b6a73e7c-be8e-47fb-851d-49509fae28a9'
            , ('email') : 'anastasiusmanurung@gmail.com', ('newPass') : 'tes123', ('idUser') : '64e7cbc9-30d7-4d02-9046-6dafa0112a48'
            , ('name') : 'anas', ('nip') : '1234', ('idRole') : '26dfb4b3-583d-4615-bef4-e0abae034951', ('code') : 'AGT'
            , ('idCompany') : '94077080-9835-42a0-aeaf-9e0dc5a9b8b5', ('companyName') : 'Karuhun']))

WS.verifyResponseStatusCode(response, 200)

WS.sendRequest(findTestObject('Accounts/Get Account', [('email') : 'superadmin']))

