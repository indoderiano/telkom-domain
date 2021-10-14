const express = require('express')
const { RouterApi } = require('./RouterApi')
const { RouterApp } = require('./RouterApp')
const { RouterSettings } = require('./RouterSettings')

const router = express.Router()

router.use('/api', RouterApi)
router.use('/applications', RouterApp)
router.use('/tenant', RouterSettings)

module.exports={
    router
}