import React from "react";
import Dashboard from "../src/pages/Dashboard";
import { getVulnerabilities } from "../src/lib/api";
import { render, screen, fireEvent } from "@testing-library/react";
import "@testing-library/jest-dom/extend-expect";

