import 'url-search-params-polyfill';
import React, {useState, useContext} from "react";
import {render as reactRender} from 'react-dom';
import * as Sentry from "@sentry/browser";
import {State} from "./app/state";
