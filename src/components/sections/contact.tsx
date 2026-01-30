"use client";
import React from "react";
import { config } from "@/data/config";
import { SectionHeader } from "./section-header";
import SectionWrapper from "../ui/section-wrapper";
import { Button } from "../ui/button";
import Link from "next/link";
import { SiGithub } from "react-icons/si";
import { Mail } from "lucide-react";

const ContactSection = () => {
  return (
    <SectionWrapper id="contact" className="min-h-screen max-w-7xl mx-auto">
      <SectionHeader
        id="contact"
        className="relative mb-14"
        title={
          <>
            GET IN <br />
            TOUCH
          </>
        }
      />
      <div className="flex flex-col items-center gap-6 mx-4 mt-10 md:mt-20">
        <p className="text-center text-lg text-muted-foreground max-w-md">
          Want to chat? Feel free to reach out via email or check out my GitHub!
        </p>
        <div className="flex flex-col sm:flex-row gap-4">
          <Link href={`mailto:${config.email}`}>
            <Button size="lg" className="gap-2">
              <Mail size={20} />
              {config.email}
            </Button>
          </Link>
          <Link href={config.social.github} target="_blank">
            <Button size="lg" variant="outline" className="gap-2">
              <SiGithub size={20} />
              GitHub
            </Button>
          </Link>
        </div>
      </div>
    </SectionWrapper>
  );
};
export default ContactSection;
