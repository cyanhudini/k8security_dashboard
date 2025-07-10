import EmailBar from "../src/components/EmailBar";
import { render, screen, fireEvent } from "@testing-library/react";
import '@testing-library/jest-dom';
import { getReceiverEmails, addReceiverEmail, setEmailStatus } from "../src/lib/api";
import { describe, test, expect, beforeEach, jest } from '@jest/globals';
jest.mock("../src/lib/api");

let mockData = [
    { id: 1, email: "dontMock@me.de", status: "active" },
    { id: 2, email: "neverFound@data.base", status: "inactive" },
    { id: 3, email: "EvilTwin@d.de", status: "active" }
]

describe("EmailBar Component", () => {
    beforeEach(() => {
        jest.clearAllMocks();
    })
    test("should get all emails", async () => {
        getReceiverEmails.mockResolvedValue(mockData);
        render(<EmailBar />);

        //i want to mock this  
        expect(getReceiverEmails).toHaveBeenCalledTimes(1);
        expect(await screen.findByText("dontMock@me.de")).toBeInTheDocument();

        
    });
    test("should add a new email", async () => {
        getReceiverEmails.mockResolvedValue(mockData);
        addReceiverEmail.mockResolvedValue({ id: 4, email: "new_test@adl,com", status: "active" });
        render(<EmailBar />);
        
        const input = screen.getByRole("textbox", { name: /email/i });
        const button = screen.getByRole("button", { name: /add email/i });
        fireEvent.change(input, { target: { value: "new_email@dddd.de" } });
        fireEvent.click(button);
        expect(addReceiverEmail).toHaveBeenCalledWith("new_email@dddd.de");
        

    })
})